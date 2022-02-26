use std::collections::HashMap;

use lentsys::ppu::attr::AttrSet;
pub mod attract_mode;
pub mod biathlon;
pub mod cutscene;
pub mod event_select;
pub mod title_screen;
pub mod medal_cere;
pub mod victory;
pub mod menu;
pub mod nation_select;
pub mod native;
pub mod sounds;
pub mod player;

pub struct SpyderEvent {
  pub start_line: [u16; 2],
  pub targets: Vec<Target>,
  pub time_limit: f32,
}

#[derive(Debug)]
pub struct SpyderEventRecord {
  pub event: String,
  pub time_remaining: f32,
}

pub struct SpyderState {
  pub events: HashMap<String, SpyderEvent>,
  pub results: HashMap<String, SpyderEventRecord>,
}

pub struct BuglympicsEvent {
  pub start_line: [u16; 2],
  pub finish_line: [u16; 2],
}

pub struct BuglympicsState {
  pub nation: String,
  pub events: HashMap<String, BuglympicsEvent>,
  pub medals: HashMap<String, MedalStanding>,
}

#[derive(Debug)]
pub struct BuglympicsEventRecord {
  pub nation: String,
  pub event: String,
  pub time: f32,
}

#[derive(Debug)]
pub struct MedalStanding {
  pub event: String,
  pub medals: Vec<BuglympicsEventRecord>,
}

impl MedalStanding {
  pub fn check_result(&mut self, record: BuglympicsEventRecord) {
    self.medals.push(record);
    self
      .medals
      .sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    self.medals.drain(3..);
  }
}

pub struct Target {
  pub entity_id: usize,
  pub hit: bool,
  pub transform: lentsys::ecs::components::transform::Transform,
  pub collider: lentsys::ecs::components::collision::BoxCollider,
  pub anim: lentsys::ecs::components::shape::AnimatedSprite
}

impl Default for Target {
  fn default() -> Self {
    Self {
      entity_id: 0,
      hit: false,
      transform: lentsys::ecs::components::transform::Transform::new(0, 0.0, 0.0),
      collider: lentsys::ecs::components::collision::BoxCollider {
        entity_id: 0,
        top: -8.0,
        bottom: 24.0,
        left: -8.0,
        right: 24.0,
        collision: false,
        collided: vec![],
        map_collided: vec![],
      },
      anim: lentsys::ecs::components::shape::AnimatedSprite {
        entity_id: 0,
        sprite_id: 1,
        tile_set_id: 3,
        tile_height: 16,
        tile_width: 16,
        tile_range: [0, 0],
        palette_range: [0, 0],
        rate: 4,
        counter: 0,
        played: false,
        frame_tile_id: vec![87],
        frame_palette_id: vec![3],
      },
    }
  }
}

impl Target {
  fn init(&mut self, bus: &mut lentsys::lentsys::LentSysBus){

    self
      .anim
      .add_to_sprites(&mut bus.ppu.sprites, &self.transform);

    self.anim.sprite_id = bus.ppu.sprites.len() - 1;

    bus.ppu.sprites[self.anim.sprite_id].scene_x = self.transform.scene_x as u16;
    bus.ppu.sprites[self.anim.sprite_id].scene_y = self.transform.scene_y as u16;

  }

  fn update(&mut self, bus: &mut lentsys::lentsys::LentSysBus) {
    bus.ppu.sprites[self.anim.sprite_id].scene_x = self.transform.scene_x as u16;
    bus.ppu.sprites[self.anim.sprite_id].scene_y = self.transform.scene_y as u16;
  }
}

pub struct SceneMap {
  pub scene: usize,
  pub both_complete: bool,
  pub bl_tm_ts: HashMap<usize, usize>,   // tile_map, tile_set
  pub bl_tm_pal: HashMap<usize, usize>,  // tile_map, palette
  pub bl_sp_ts: HashMap<usize, usize>,   // sprite, tile_set
  pub bl_sp_pal: HashMap<usize, usize>,  // sprite, palette
  pub spy_tm_ts: HashMap<usize, usize>,  // tile_map, tile_set
  pub spy_tm_pal: HashMap<usize, usize>, // tile_map, palette
  pub spy_sp_ts: HashMap<usize, usize>,  // sprite, tile_set
  pub spy_sp_pal: HashMap<usize, usize>, // sprite, palette
}

impl Default for SceneMap {
  fn default() -> Self {
    SceneMap {
      scene: 5,
      both_complete: false,
      bl_tm_ts: vec![(0, 0)].into_iter().collect(),
      bl_tm_pal: vec![(0, 0), (1, 1)].into_iter().collect(),
      bl_sp_ts: vec![(0, 2),].into_iter().collect(),
      bl_sp_pal: vec![(0, 2),].into_iter().collect(),
      spy_tm_ts: vec![(0, 3)].into_iter().collect(),
      spy_tm_pal: vec![(0, 3)].into_iter().collect(),
      spy_sp_ts: vec![(0, 4)].into_iter().collect(),
      spy_sp_pal: vec![(0, 4)].into_iter().collect(),
    }
  }
}

pub struct WorldState {
  pub gravity: f32,
  pub collision_set: AttrSet,
}

#[derive(Debug)]
pub enum GameMode {
  Spyder,
  Buglympics,
}

pub struct GameState {
  pub game: GameMode,
  pub swap_cooldown: u32,
  pub current_scene: usize,
  pub event: String,
  pub last_event_success: bool,
  pub events: HashMap<String, SceneMap>,
  pub world: WorldState,
  pub buglympics: BuglympicsState,
  pub spyder: SpyderState,
}

impl GameState {
  pub fn set_buglympics(&mut self, bus: &mut lentsys::lentsys::LentSysBus) {
    if !self.events.contains_key(&self.event) {
      return;
    }

    let mapping = self.events.get(&self.event).unwrap();

    for (tm_id, ts_id) in mapping.bl_tm_ts.iter() {
      bus.ppu.tile_maps[*tm_id].tile_set_id = *ts_id;
    }

    for (tm_id, pal_id) in mapping.bl_tm_pal.iter() {
      bus.ppu.tile_maps[*tm_id].palette_id = *pal_id;
    }

    for (sp_id, ts_id) in mapping.bl_sp_ts.iter() {
      bus.ppu.sprites[*sp_id].tile_set_id = *ts_id;
    }

    for (sp_id, pal_id) in mapping.bl_sp_pal.iter() {
      bus.ppu.sprites[*sp_id].palette_id = *pal_id as u16;
    }

    self.game = GameMode::Buglympics;
  }

  pub fn set_spyder(&mut self, bus: &mut lentsys::lentsys::LentSysBus) {
    if !self.events.contains_key(&self.event) {
      return;
    }

    let mapping = self.events.get(&self.event).unwrap();

    for (tm_id, ts_id) in mapping.spy_tm_ts.iter() {
      bus.ppu.tile_maps[*tm_id].tile_set_id = *ts_id;
    }

    for (tm_id, pal_id) in mapping.spy_tm_pal.iter() {
      bus.ppu.tile_maps[*tm_id].palette_id = *pal_id;
    }

    for (sp_id, ts_id) in mapping.spy_sp_ts.iter() {
      bus.ppu.sprites[*sp_id].tile_set_id = *ts_id;
    }

    for (sp_id, pal_id) in mapping.spy_sp_pal.iter() {
      bus.ppu.sprites[*sp_id].palette_id = *pal_id as u16;
    }

    self.game = GameMode::Spyder;
  }

  pub fn check_game(&mut self, bus: &mut lentsys::lentsys::LentSysBus) {
    match &self.game {
      GameMode::Buglympics => {
        self.set_buglympics(bus);
      }
      GameMode::Spyder => {
        self.set_spyder(bus);
      }
    }
  }

  pub fn swap_game(&mut self, bus: &mut lentsys::lentsys::LentSysBus) {
    if self.swap_cooldown > 10 {
      self.swap_cooldown = 0;
      match &self.game {
        GameMode::Buglympics => {
          self.set_spyder(bus);
        }
        GameMode::Spyder => {
          self.set_buglympics(bus);
        }
      }
    }
  }
}
