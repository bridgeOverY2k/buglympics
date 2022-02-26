use std::collections::HashSet;

use lentsys::ecs::components::collision::BoxCollider;
use lentsys::ecs::components::collision::Collide;
use lentsys::ecs::components::collision::Ray;
use lentsys::ecs::components::shape::AnimatedSprite;
use lentsys::ecs::components::transform::Transform;
use lentsys::ppu::attr::TileAttr;

use lentsys::lentsys::LentSysBus;

use crate::game::state::{GameMode, GameState};
use crate::game::sounds;
use crate::game::input::{InputCode};

#[derive(Debug)]
pub enum PlayerState {
  Standing,
  Walking,
  Running,
  Jumping,
}

pub struct Projectile {
  pub max_distance: f32,
  pub distance_traveled: f32,
  pub expired: bool,
  pub speed: f32,
  pub direction: [f32; 2],
  pub transform: Transform,
  pub sensor: Ray,
  pub anim: AnimatedSprite,
}

impl Default for Projectile {
  fn default() -> Self {
    Self {
      max_distance: 300.0,
      distance_traveled: 0.0,
      expired: false,
      speed: 8.0,
      direction: [1.0, 0.0],
      transform: Transform::new(0, 0.0, 0.0),
      sensor: Ray::new([0.0, 0.0], [1.0, 0.0], 1.0, 1),
      anim: AnimatedSprite {
        entity_id: 0,
        sprite_id: 1,
        tile_set_id: 4,
        tile_height: 16,
        tile_width: 16,
        tile_range: [0, 2],
        palette_range: [0, 0],
        rate: 0,
        counter: 0,
        played: false,
        frame_tile_id: (48..51).collect(),
        frame_palette_id: vec![4],
      },
    }
  }
}

impl Projectile {
  fn update(&mut self, bus: &mut LentSysBus, state: &mut GameState) {
    if self.distance_traveled > self.max_distance {
      self.expired = true;
      bus.ppu.sprites[self.anim.sprite_id].hide = true;
    } else {
      self
        .transform
        .translate(self.direction[0] * self.speed, 0.0);
      self.distance_traveled += self.direction[0].abs() * self.speed;

      let mut sensor = Ray::new(
        [self.transform.scene_x, self.transform.scene_y],
        self.direction,
        self.speed,
        self.speed as u8,
      );
      let (_mc_idx, mut hit) = get_nearest_map_collision(&mut sensor, bus, state);

      let event = state.spyder.events.get_mut(&state.event).unwrap();

      for tgt in event.targets.iter_mut() {
        sensor.check_box_collision([0.0, 0.0], &tgt.collider, &tgt.transform);
        //println!("{:?}", sensor.collided);
        if sensor.collided.len() > 0 {
          tgt.hit = true;
          hit = true;
          bus.ppu.sprites[tgt.anim.sprite_id].hide = true;
          //println!("TARGET HIT!");
        }
      }

      if hit {
        self.expired = true;
        bus.ppu.sprites[self.anim.sprite_id].hide = true;
      } else {
        bus.ppu.sprites[self.anim.sprite_id].scene_x = self.transform.scene_x as u16;
        bus.ppu.sprites[self.anim.sprite_id].scene_y = self.transform.scene_y as u16;
      }
    }
  }
}

pub struct Launcher {
  pub cooldown: u8,
  pub max_projectiles: u8,
  pub ammo: u8,
  pub projectile_tile: usize,
  pub projectile_speed: f32,
  pub projectiles: Vec<Projectile>,
}

impl Default for Launcher {
  fn default() -> Self {
    Self {
      cooldown: 0,
      max_projectiles: 3,
      ammo: 255,
      projectile_tile: 0,
      projectile_speed: 8.0,
      projectiles: vec![],
    }
  }
}

impl Launcher {
  fn fire(&mut self, start_pos: [f32; 2], direction: [f32; 2], bus: &mut LentSysBus) {
    self.projectiles = self.projectiles.drain(..).filter(|p| !p.expired).collect();
    if self.ammo < 1 {
      return;
    }

    if self.projectiles.len() > self.max_projectiles as usize {
      self.projectiles = self.projectiles.drain(0..1).collect();
    }

    if self.cooldown > 8 && self.projectiles.len() < self.max_projectiles as usize {
      let mut proj = Projectile {
        direction,
        transform: Transform::new(0 as usize, start_pos[0], start_pos[1]),
        sensor: Ray::new(
          start_pos,
          direction,
          self.projectile_speed,
          self.projectile_speed as u8,
        ),
        ..Projectile::default()
      };

      proj
        .anim
        .add_to_sprites(&mut bus.ppu.sprites, &proj.transform);

      proj
        .anim
        .jump_to(self.projectile_tile, &mut bus.ppu.sprites);

      //println!("{} {}", bus.ppu.sprites[proj.anim.sprite_id].tile_id, self.projectile_tile);

      // sound
      sounds::play_effect(bus, sounds::SFX::Fire, 8000);

      proj.anim.sprite_id = bus.ppu.sprites.len() - 1;
      self.projectiles.push(proj);

      self.cooldown = 0;
      self.ammo -= 1;
    }
  }
}


pub struct Player {
  pub entity_id: usize,
  pub dead: bool,
  pub vel_x: f32,
  pub vel_y: f32,
  pub jumping: bool,
  pub running: bool,
  pub grounded: bool,
  pub blocked: [bool; 2],
  pub finished: bool,
  pub player_state: PlayerState,
  pub jump_force: f32,
  pub slope: f32,
  pub slope_accel: f32,
  pub accel_rate: f32,
  pub decel_rate: f32,
  pub walk_speed: f32,
  pub run_speed: f32,
  pub air_control: f32,
  pub launcher: Launcher,
  pub transform: Transform,
  pub collider: BoxCollider,
  pub anim: AnimatedSprite,
}

impl Player {
  pub fn new(entity_id: usize, start_pos: [u16; 2]) -> Player {
    let transform = Transform {
      entity_id: entity_id,
      scene_x: start_pos[0] as f32,
      scene_y: start_pos[1] as f32,
      translate_x: 0.0,
      translate_y: 0.0,
    };
    Player {
      entity_id: entity_id,
      dead: false,
      vel_x: 0.0,
      vel_y: 0.0,
      accel_rate: 0.1,
      decel_rate: 0.9,
      walk_speed: 3.0,
      run_speed: 7.0,
      jump_force: 10.0,
      air_control: 0.75,
      slope: 0.0,
      slope_accel: 1.0,
      jumping: false,
      running: false,
      grounded: false,
      blocked: [false, false],
      finished: false,
      player_state: PlayerState::Jumping,
      launcher: Launcher {
        ..Launcher::default()
      },
      transform,
      collider: BoxCollider {
        entity_id: entity_id,
        top: 0.0,
        bottom: 48.0,
        left: 0.0,
        right: 32.0,
        collision: false,
        collided: vec![],
        map_collided: vec![],
      },
      anim: AnimatedSprite {
        entity_id: entity_id,
        sprite_id: 0,
        tile_set_id: 2,
        tile_height: 48,
        tile_width: 32,
        tile_range: [1, 5],
        palette_range: [0, 0],
        rate: 4,
        counter: 0,
        played: false,
        frame_tile_id: (0..9).collect(),
        frame_palette_id: vec![2],
      },
    }
  }

  pub fn init(&mut self, bus: &mut LentSysBus) {
    //self.shape.add_to_sprites(&mut bus.ppu.sprites, &self.transform);
    //self.shape.shape[self.shape.current_frame].show(&mut bus.ppu.sprites);
    self
      .anim
      .add_to_sprites(&mut bus.ppu.sprites, &self.transform);

    //self
    //.skis.init(bus);
  }

  pub fn update(&mut self, bus: &mut LentSysBus, keys: &HashSet<InputCode>, state: &mut GameState) {
    /*
      Roughly, the order should be
      - Inputs
      - Transform
      - Collisions
      - Animation
    */
    //println!("{} {}", self.slope_accel,  ((self.slope_accel - 0.75) / 0.75 * 0.09));
    self.vel_x *= self.decel_rate + ((self.slope_accel - 0.75) / 0.75 * 0.09);
    let mut move_speed = self.walk_speed;
    self.anim.rate = 6;

    if self.launcher.cooldown < 10 {
      self.launcher.cooldown += 1;
    }

    if keys.contains(&InputCode::Fire) {
      match &state.game {
        GameMode::Buglympics => {
          move_speed = self.run_speed;
          self.anim.rate = 8;
        }
        GameMode::Spyder => {
          let direction = if bus.ppu.sprites[self.anim.sprite_id].reverse_x {
            -1.0
          } else {
            1.0
          };
          self.launcher.fire(
            [self.transform.scene_x, self.transform.scene_y],
            [direction, 0.0],
            bus,
          );
        }
      }
    }

    match &self.player_state {
      PlayerState::Jumping => {
        if keys.contains(&InputCode::Right) {
          self.vel_x += move_speed * self.accel_rate * self.air_control;
          bus.ppu.sprites[self.anim.sprite_id].reverse_x = false;
        }

        if keys.contains(&InputCode::Left) {
          self.vel_x += -move_speed * self.accel_rate * self.air_control;
          bus.ppu.sprites[self.anim.sprite_id].reverse_x = true;
        }

        self.vel_y = if self.vel_y >= 10.0 {
          10.0
        } else {
          self.vel_y + state.world.gravity
        };
      }
      PlayerState::Standing => {
        self.vel_y *= 0.0;

        if keys.contains(&InputCode::Right) {
          self.player_state = PlayerState::Walking;
          self.vel_x += move_speed * self.slope_accel * self.accel_rate;
          bus.ppu.sprites[self.anim.sprite_id].reverse_x = false;
        }
        if keys.contains(&InputCode::Left) {
          self.player_state = PlayerState::Walking;
          self.vel_x += -move_speed * self.slope_accel * self.accel_rate;
          bus.ppu.sprites[self.anim.sprite_id].reverse_x = true;
        }

        if keys.contains(&InputCode::Jump) {
          self.player_state = PlayerState::Jumping;
          self.vel_y = -self.jump_force;
          match &state.game {
            crate::game::state::GameMode::Buglympics => {
              sounds::play_effect(bus, sounds::SFX::JumpA, 800);
            }
            crate::game::state::GameMode::Spyder => {
              sounds::play_effect(bus, sounds::SFX::JumpB, 800);
            }
          }
        }
      }
      _ => self.player_state = PlayerState::Standing,
    }

    self.vel_x = if self.vel_x.abs() >= self.run_speed {
      self.run_speed * self.vel_x.signum()
    } else {
      self.vel_x
    };

    // APPLY TRANSFORM
    //println!("velocity x {}, velocity y {}", self.vel_x, self.vel_y);
    if (self.blocked[1] && self.vel_x > 0.0) || (self.blocked[0] && self.vel_x < 0.0) {
      self.player_state = PlayerState::Standing;
      self.transform.translate(0.0, self.vel_y);
    } else {
      self.transform.translate(self.vel_x, self.vel_y);
    }

    // COLLISION
    self.check_side_collision(bus, &state);
    self.grounded = false;
    self.check_ground_collision(bus, &state);

    // Buglympics - Check if crossed finish line
    match &state.game {
      crate::game::state::GameMode::Buglympics => {
        let bl_event = state.buglympics.events.get(&state.event).unwrap();
        if (self.transform.scene_x - bl_event.finish_line[0] as f32).abs() < 16.0
          && (self.transform.scene_y - bl_event.finish_line[1] as f32).abs() < 48.0
        {
          self.finished = true;
        }
      }
      _ => {}
    }

    if !self.grounded {
      self.slope = 0.0;
      self.player_state = PlayerState::Jumping;
    }

    // PROJECTILES
    for proj in self.launcher.projectiles.iter_mut() {
      proj.update(bus, state);
    }

    match &self.player_state {
      PlayerState::Walking => {
        if self.slope_accel > 1.0 {
          self.anim.tile_range = [5, 5];
          self.anim.jump_to(5, &mut bus.ppu.sprites);
          sounds::play_effect(bus, sounds::SFX::Ski, 800);
        } else {
          self.anim.tile_range = [1, 5];
          self.anim.advance_tile(&mut bus.ppu.sprites);
        }
        self.player_state = PlayerState::Standing;
      }
      PlayerState::Jumping => {
        if self.grounded {
          self.anim.jump_to(1, &mut bus.ppu.sprites);
          self.player_state = PlayerState::Standing;
        } else {
          self.anim.tile_range = [6, 6];
          self.anim.jump_to(6, &mut bus.ppu.sprites);
        }
      }
      PlayerState::Standing => {
        self.anim.tile_range = [0, 0];
        self.anim.jump_to(0, &mut bus.ppu.sprites);
      }
      _ => {}
    }

    if self.transform.scene_y > 205.0 * 16.0 {
      self.transform.scene_x = 100.0;
      self.transform.scene_y = 100.0;
    }

    //self
    //  .skis
    //  .update(&self.transform, bus.ppu.sprites[self.anim.sprite_id].reverse_x, bus, &state.game);

    bus.ppu.sprites[self.anim.sprite_id].scene_x = self.transform.scene_x as u16;
    bus.ppu.sprites[self.anim.sprite_id].scene_y = self.transform.scene_y as u16;
  }

  fn check_ground_collision(&mut self, bus: &mut LentSysBus, state: &GameState) {
    // if jumping, early exit;
    if self.vel_y < 0.0 {
      return;
    }

    // cast 8 pixels below
    let mut sensors = vec![Ray::new(
      [
        self.transform.scene_x + (self.collider.left + self.collider.right) * 0.5,
        self.transform.scene_y + self.collider.bottom,
      ],
      [0.0, 1.0],
      8.0,
      8,
    )];

    // find tile_map collisions
    let (sensor_idx, mc_idx, hit) = self.get_nearest_map_collision(&mut sensors, bus, state);

    if hit {
      self.grounded = hit;
      let hit_mc = &sensors[sensor_idx].map_collided[mc_idx];
      self.slope = self.find_surface_height(hit_mc.tile_id, hit_mc.point[0], state);
      self.slope_accel = self.find_surface_angle(hit_mc.tile_id, hit_mc.point[0], state);

      //if it is a solid tile, check above
      if self.slope == 16.0 {
        let tile_above = hit_mc.map_loc_id
          - bus.ppu.tile_maps[state.world.collision_set.tile_set_id].columns as usize;
        let tile_type_above =
          bus.ppu.tile_maps[state.world.collision_set.tile_set_id].data[tile_above];
        self.slope += self.find_surface_height(tile_type_above as usize, hit_mc.point[0], state);
        self.slope_accel = self.find_surface_angle(hit_mc.tile_id, hit_mc.point[0], state);
      }

      // if hit detected early (up to 8 pixels), correct
      let err = hit_mc.point[1] - sensors[sensor_idx].origin[1];

      /*println!("searched from {:?} detected neared hit at {:?} which overlaps {:?}",
      sensors[sensor_idx].origin,
      hit_mc.point,
      sensors[sensor_idx].map_collided[mc_idx].overlap[1] + err
      );*/
      self.transform.translate(
        0.0,
        &sensors[sensor_idx].map_collided[mc_idx].overlap[1] + err + (16.0 - self.slope),
      );
    }
  }

  pub fn check_side_collision(&mut self, bus: &LentSysBus, state: &GameState) {
    // check moving direction
    let mut side: usize = 1;
    let mut offset: f32 = self.collider.right;

    if self.vel_x < 0.0 {
      side = 0;
      offset = self.collider.left;
    }

    let mut sensors = vec![Ray::new(
      [
        self.transform.scene_x + offset,
        self.transform.scene_y + 16.0,
      ],
      [self.vel_x.signum(), 0.0],
      4.0,
      4,
    )];

    let (sensor_idx, mc_idx, hit) = self.get_nearest_map_collision(&mut sensors, bus, state);
    if hit {
      let hit_mc = &sensors[sensor_idx].map_collided[mc_idx];
      let surface = self.find_surface_height(hit_mc.tile_id, hit_mc.point[0], state);
      let err = hit_mc.point[0] - sensors[sensor_idx].origin[0];
      /*println!("origin {:?} collision {:?} distance {} err {}",
      sensors[sensor_idx].origin,
      sensors[sensor_idx].map_collided[mc_idx],
      distance,
      err);*/
      if surface == 16.0 {
        self.blocked[side] = true;
        self.vel_x = 0.0;
        self.transform.translate(err, 0.0);
      }
    } else {
      self.blocked[side] = false;
    }
  }

  fn find_surface_height(&mut self, tile_id: usize, x_overlap: f32, state: &GameState) -> f32 {
    let tile_attr = state.world.collision_set.tiles.get(&tile_id);
    match tile_attr {
      Some(TileAttr::Angle(ang)) => {
        let tables = [
          [
            16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16, 16,
          ],
          [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
          [8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 15, 15],
          [0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7],
          [
            13, 13, 14, 14, 14, 14, 14, 14, 14, 14, 15, 15, 15, 16, 16, 16,
          ],
        ];
        let mut table = tables[ang.abs() as usize];
        let idx = (x_overlap % 16.0).floor() as usize;

        if ang < &0 {
          table.reverse();
        };

        return table[idx] as f32;
      }
      Some(_) => return 16.0,
      None => return 0.0,
    }
  }

  pub fn find_surface_angle(&mut self, tile_id: usize, _x_overlap: f32, state: &GameState) -> f32 {
    let tile_attr = state.world.collision_set.tiles.get(&tile_id);
    match tile_attr {
      Some(TileAttr::Angle(ang)) => {
        let accel_table = [0.0, 0.5, 0.25, 0.25, 0.0];
        return self.vel_x.signum() * -ang.signum() as f32 * accel_table[ang.abs() as usize] + 1.0;
      }
      _ => {
        return 1.0;
      }
    }
  }

  pub fn get_nearest_map_collision(
    &mut self,
    sensors: &mut Vec<Ray>,
    bus: &LentSysBus,
    state: &GameState,
  ) -> (usize, usize, bool) {
    let mut hit = false;
    let mut sensor_idx = 0;
    let mut mc_idx = 0;
    let mut nearest_collision_distance = 1.0e6;
    for (i, s) in sensors.iter_mut().enumerate() {
      s.check_tile_collision(
        [0.0, 0.0],
        &bus.ppu.tile_maps[state.world.collision_set.tile_set_id],
      );
      for (j, mc) in s.map_collided.iter().enumerate() {
        if state.world.collision_set.tiles.contains_key(&mc.tile_id) {
          let distance_from_collision = (s.origin[1] - mc.point[1]).abs();
          if distance_from_collision < nearest_collision_distance {
            nearest_collision_distance = distance_from_collision;
            sensor_idx = i;
            mc_idx = j;
          }

          hit = true;
        }
      }
    }

    return (sensor_idx, mc_idx, hit);
  }
}

pub fn get_nearest_map_collision(
  sensor: &mut Ray,
  bus: &LentSysBus,
  state: &GameState,
) -> (usize, bool) {
  let mut hit = false;
  let mut mc_idx = 0;
  let mut nearest_collision_distance = 1.0e6;

  sensor.check_tile_collision(
    [0.0, 0.0],
    &bus.ppu.tile_maps[state.world.collision_set.tile_set_id],
  );
  for (j, mc) in sensor.map_collided.iter().enumerate() {
    //println!("{:?}", mc);
    if state.world.collision_set.tiles.contains_key(&mc.tile_id) {
      let distance_from_collision = (sensor.origin[1] - mc.point[1]).abs();
      if distance_from_collision < nearest_collision_distance {
        nearest_collision_distance = distance_from_collision;
        mc_idx = j;
      }

      hit = true;
    }
  }

  return (mc_idx, hit);
}
