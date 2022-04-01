use lentsys::lentsys::LentSysBus;
use lentsys::ppu::attr::TileAttr;
use lentsys::ui::text::Text;
use lentsys::ui::text::TextBox;
use lentsys::game_pak::scene::SceneState;

use crate::game::input::InputCode;
use crate::game::player::Player;
use crate::game::sounds::prepare_effects;
use crate::game::state::BuglympicsEventRecord;
use crate::game::state::GameState;

pub fn init(bus: &mut LentSysBus, state: &mut GameState) {
    // set timers and end state
    state.scene_frames = 0;
    state.bl_timer = 0.0;
    state.spy_timer = state.spyder.events[&state.event].time_limit;
    state.bl_finished = false;
    state.spy_finished = false;

    TextBox::new(
        String::from("00:00.00"),
        320.0 - (16.0 * 8.0),
        224.0,
        String::from("start_font_small"),
        String::from("start_font_small"),
        8,
        Some(9),
        Some(2),
    )
    .to_tilemap(bus);

    // Palette swap for nation
    if &state.buglympics.nation == "EAST ARACHNYLVANIA" {
        bus.ppu.palettes[2].data[4] = [162, 0, 0, 255];
        state.player.launcher.projectile_tile = 1;
    } else if &state.buglympics.nation == "REP. OF WORMSTRALIA" {
        bus.ppu.palettes[2].data[4] = [22, 111, 2, 255];
        state.player.launcher.projectile_tile = 2;
    }

    // Initialize Player
    let level = state.buglympics.events.get(&state.event).unwrap();

    state.player = Player::new(1, level.start_line);
    state.player.init(bus);

    // Initialize Spyder targets
    {
        data_entity_handler(
            &bus.game_pak.scenes[state.current_scene].data_entities,
            state,
        );
        let event = state.spyder.events.get_mut(&state.event).unwrap();
        for tgt in event.targets.iter_mut() {
            tgt.init(bus);
        }
    }

    state.hit_count = 0;

    // Set tile attributes for collision and slope
    set_tile_attrs(state, true);

    // Set camera boundaries and initial position
    bus.ppu.screen_state.map_max_x = bus.ppu.tile_maps[0].columns * bus.ppu.tile_maps[0].tile_width;
    bus.ppu.screen_state.map_max_y = bus.ppu.tile_maps[0].rows * bus.ppu.tile_maps[0].tile_height;
    bus.ppu.screen_state.scroll_y = 200;

    state.check_game(bus);

    state.last_event_success = false;

    // sounds
    prepare_effects(bus);

    
}

pub fn update(bus: &mut LentSysBus, state: &mut GameState) {
    /*
    Update timer and check finished conditions
    */

    let time_delta = 0.01667; // assume 1 frame constant time

    match state.game {
        crate::game::state::GameMode::Buglympics => {
            if !state.bl_finished {
                state.bl_timer += time_delta;
            }

            // Check if medal worthy
            if state.player.finished && !state.bl_finished {
                //println!("Finished at : {}", &clock_time);
                let medals = state.buglympics.medals.get_mut(&state.event).unwrap();
                medals.check_result(BuglympicsEventRecord {
                    nation: state.buglympics.nation.to_string(),
                    event: state.event.to_string(),
                    time: state.bl_timer,
                });
                state.bl_finished = true;
            }
        }
        crate::game::state::GameMode::Spyder => {
            if !state.spy_finished {
                state.spy_timer -= time_delta;
            }

            
            state.hit_count = 0;
            let mut all = true;
            let event = state.spyder.events.get_mut(&state.event).unwrap();
            // Are all targets hit?
            for tgt in event.targets.iter_mut() {
                state.hit_count += tgt.hit as u8;
                tgt.update(bus);
                if all {
                    all = tgt.hit;
                }
            }

            if all && !state.spy_finished {
                state.spyder.results.insert(
                    state.event.to_string(),
                    crate::game::state::SpyderEventRecord {
                        event: state.event.to_string(),
                        time_remaining: state.spy_timer,
                    },
                );

                state.spy_finished = true;
            }

            // Times up
            if state.spy_timer < 0.0 && !state.spy_finished {
                state.last_event_success = false;

                // set this scene as complete
                bus.game_pak.scenes[state.current_scene].state = SceneState::COMPLETE;

                state.current_scene = 4;
            }
        }
    }

    display_timer(state, bus);

    /*
    Event Complete?
    */

    if state.bl_finished && state.spy_finished {
        state.events.get_mut(&state.event).unwrap().both_complete = true;
        state.last_event_success = true;

        // set this scene as complete
        bus.game_pak.scenes[state.current_scene].state = SceneState::COMPLETE;

        state.current_scene = 4;
    }

    /*
    Update game
    */

    let next_screen_pos = [
        (state.player.transform.scene_x - 100.0) as i16,
        (state.player.transform.scene_y - 100.0) as i16,
    ];


    

    // Player
    let finish_line = state.buglympics.events.get(&state.event).unwrap().finish_line;

    state.player.update(
        bus, 
        &state.inputs, 
        &state.game, 
        &state.world,
        &mut state.spyder.events.get_mut(&state.event).unwrap().targets,
        finish_line
    );

    // Camera
    bus.ppu
        .screen_state
        .lerp(next_screen_pos, time_delta * 10.0);

    // catch if fall off bottom of map
    if state.player.transform.scene_y + 96.0 > bus.ppu.screen_state.map_max_y as f32 {
        state.last_event_success = false;

        // set this scene as complete
        bus.game_pak.scenes[state.current_scene].state = SceneState::COMPLETE;

        state.current_scene = 4;
    }

    // Game Hot Swap
    if state.inputs.contains(&InputCode::Swap) {
        state.swap_game(bus);
        set_tile_attrs(state, false);
    }
    state.swap_cooldown += 1;

    state.scene_frames += 1;
}

pub fn display_timer(state: &mut GameState, bus: &mut LentSysBus) {
    let clock_time: f32;
    let timer_map_idx = bus.ppu.tile_maps.len() - 1;

    match state.game {
        crate::game::state::GameMode::Buglympics => {
            state.hit_text = String::from("");
            clock_time = state.bl_timer;
        }
        crate::game::state::GameMode::Spyder => {
            let event = state.spyder.events.get_mut(&state.event).unwrap();
            state.hit_text = format!("HIT {}/{}", state.hit_count, event.targets.len());
            clock_time = state.spy_timer;
        }
    }

    let time = format!(
        "{}\n{}:{}.{}",
        state.hit_text,
        clock_time as u32 / 60,
        clock_time as u32 % 60,
        ((clock_time / clock_time.floor() - 1.0) * 1000.0) as u32
    )
    .to_string();
    bus.ppu.tile_maps[timer_map_idx].update_text(time);
}

pub fn data_entity_handler(data_entities: &Vec<lentsys::ecs::DataEntity>, state: &mut GameState) {
    use crate::game::state::Target;
    for ent in data_entities.iter() {
        match ent.data_entity_type.as_str() {
            "target" => {
                let mut scene_x = 0.0;
                let mut scene_y = 0.0;
                for dc in ent.data_components.iter() {
                    match dc.param_name.as_str() {
                        "scene_x" => scene_x = dc.param_value.parse::<f32>().unwrap(),
                        "scene_y" => scene_y = dc.param_value.parse::<f32>().unwrap(),
                        _ => {}
                    }
                }

                let target = Target {
                    transform: lentsys::ecs::components::transform::Transform::new(
                        0, scene_x, scene_y,
                    ),
                    ..Target::default()
                };

                state
                    .spyder
                    .events
                    .get_mut(&state.event)
                    .unwrap()
                    .targets
                    .push(target);
            }
            _ => {}
        }
    }
}

// Add / set tile set attributes for collision and slope
pub fn set_tile_attrs(state: &mut GameState, initial: bool) {
    if initial {
        state
            .world
            .collision_set
            .tiles
            .insert(2, TileAttr::Angle(0));
        state
            .world
            .collision_set
            .tiles
            .insert(3, TileAttr::Angle(0));
        state
            .world
            .collision_set
            .tiles
            .insert(4, TileAttr::Angle(0));
        state
            .world
            .collision_set
            .tiles
            .insert(5, TileAttr::Angle(0));
        state
            .world
            .collision_set
            .tiles
            .insert(6, TileAttr::Angle(0));
        state
            .world
            .collision_set
            .tiles
            .insert(65, TileAttr::Angle(0));
        state
            .world
            .collision_set
            .tiles
            .insert(66, TileAttr::Angle(0));

        state
            .world
            .collision_set
            .tiles
            .insert(67, TileAttr::Angle(-1));
        state
            .world
            .collision_set
            .tiles
            .insert(82, TileAttr::Angle(0));

        state
            .world
            .collision_set
            .tiles
            .insert(68, TileAttr::Angle(1));
        state
            .world
            .collision_set
            .tiles
            .insert(83, TileAttr::Angle(0));

        state
            .world
            .collision_set
            .tiles
            .insert(69, TileAttr::Angle(-2));
        state
            .world
            .collision_set
            .tiles
            .insert(71, TileAttr::Angle(-3));
        state
            .world
            .collision_set
            .tiles
            .insert(78, TileAttr::Angle(0));
        state
            .world
            .collision_set
            .tiles
            .insert(79, TileAttr::Angle(0));

        state
            .world
            .collision_set
            .tiles
            .insert(70, TileAttr::Angle(2));
        state
            .world
            .collision_set
            .tiles
            .insert(72, TileAttr::Angle(3));
        state
            .world
            .collision_set
            .tiles
            .insert(80, TileAttr::Angle(0));
        state
            .world
            .collision_set
            .tiles
            .insert(81, TileAttr::Angle(0));

        state
            .world
            .collision_set
            .tiles
            .insert(73, TileAttr::Angle(-4));
        state
            .world
            .collision_set
            .tiles
            .insert(74, TileAttr::Angle(4));
    }

    match &state.game {
        crate::game::state::GameMode::Buglympics => {
            state.world.collision_set.tiles.remove_entry(&89);
        }
        crate::game::state::GameMode::Spyder => {
            state
                .world
                .collision_set
                .tiles
                .insert(89, TileAttr::Angle(0));
        }
    }
}
