extern crate sdl2;
use sdl2::audio::AudioQueue;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::EventPump;

use lentsys::lentsys::LentSysBus;
use lentsys::ppu::attr::TileAttr;
use lentsys::ui::text::Text;
use lentsys::ui::text::TextBox;

use std::collections::HashSet;
use std::time::Instant;

use crate::game::native::NativeVideo;
use crate::game::player::Player;
use crate::game::sounds::prepare_effects;
use crate::game::state::BuglympicsEventRecord;
use crate::game::state::GameState;

pub fn run_biathlon(
    bus: &mut LentSysBus,
    events: &mut EventPump,
    texture: &mut sdl2::render::Texture,
    vid: &mut NativeVideo,
    audio_queue: &mut AudioQueue<f32>,
    state: &mut GameState,
) {
    // Set bg color
    let bg_color = &bus.ppu.palettes[0].data[0];
    vid.canvas
        .set_draw_color(Color::RGB(bg_color[0], bg_color[1], bg_color[2]));

    // Prepare timer
    let mut last = 0.0;
    let timer = Instant::now();
    let mut bl_timer = 0.0;
    let mut spy_timer = state.spyder.events[&state.event].time_limit;
    let mut bl_finished = false;
    let mut spy_finished = false;

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

    let timer_map_idx = bus.ppu.tile_maps.len() - 1;


    let mut extra_text: String;

    let mut mt = lentsys::apu::music::MusicTracker::new(4);

    // Initialize Player
    let level = state.buglympics.events.get(&state.event).unwrap();

    let mut player = Player::new(1, level.start_line);
    player.init(bus);

    // Palette swap for nation
    if &state.buglympics.nation == "EAST ARACHNYLVANIA" {
        bus.ppu.palettes[2].data[4] = [162, 0, 0, 255];
        player.launcher.projectile_tile = 1;
    } else if &state.buglympics.nation == "REP. OF WORMSTRALIA" {
        bus.ppu.palettes[2].data[4] = [22, 111, 2, 255];
        player.launcher.projectile_tile = 2;
    }

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
    //println!("palette {:?}", bus.ppu.palettes[3].data);
    //println!("tileset {:?}", bus.ppu.tile_sets[3].data);

    // Sounds
    prepare_effects(bus);

    // Set tile attributes for collision and slope
    set_tile_attrs(state, true);

    // Set camera boundaries and initial position
    bus.ppu.screen_state.map_max_x = bus.ppu.tile_maps[0].columns * bus.ppu.tile_maps[0].tile_width;
    bus.ppu.screen_state.map_max_y = bus.ppu.tile_maps[0].rows * bus.ppu.tile_maps[0].tile_height;
    bus.ppu.screen_state.scroll_y = 200;

    state.check_game(bus);

    state.last_event_success = false;

    audio_queue.queue(&bus.apu.samples[3].play());
    audio_queue.resume();

    'biathlon: loop {
        /*
        Check for inputs
        */
        for event in events.poll_iter() {
            if let Event::Quit { .. } = event {
                println!("Exiting");
                std::process::exit(0);
            };
        }

        let keys: HashSet<Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        /*
        Check time
        */

        let clock_time: f32;
        let elapsed = timer.elapsed().as_secs_f32();
        let time_delta = elapsed - last;
        let next_screen_pos = [
            (player.transform.scene_x - 100.0) as i16,
            (player.transform.scene_y - 100.0) as i16,
        ];

        /*
        Update timer and check finished conditions
        */

        match state.game {
            crate::game::state::GameMode::Buglympics => {
                
                if !bl_finished {
                    bl_timer += time_delta;
                } 
                
                clock_time = bl_timer;
                extra_text = String::from("");

                // Check if medal worthy
                if player.finished && !bl_finished {
                    //println!("Finished at : {}", &clock_time);
                    let medals = state.buglympics.medals.get_mut(&state.event).unwrap();
                    medals.check_result(BuglympicsEventRecord {
                        nation: state.buglympics.nation.to_string(),
                        event: state.event.to_string(),
                        time: bl_timer,
                    });
                    bl_finished = true;
                }
            }
            crate::game::state::GameMode::Spyder => {

                if !spy_finished {
                    spy_timer -= time_delta;
                }

                clock_time = spy_timer;

                let mut hit_count = 0;
                let mut all = true;
                let event = state.spyder.events.get_mut(&state.event).unwrap();
                // Are all targets hit?
                for tgt in event.targets.iter_mut() {
                    hit_count += tgt.hit as u8;
                    tgt.update(bus);
                    if all {
                        all = tgt.hit;
                    }
                }

                extra_text = format!(
                    "HIT {}/{}",
                    hit_count, event.targets.len()
                  );

                if all && !spy_finished {
                    state.spyder.results.insert(
                        state.event.to_string(),
                        crate::game::state::SpyderEventRecord {
                            event: state.event.to_string(),
                            time_remaining: spy_timer,
                        },
                    );

                    spy_finished = true;
                }

                // Times up
                if clock_time < 0.0 && !spy_finished {
                    state.last_event_success = false;
                    break 'biathlon;
                }
            }
        }

        last = elapsed;

        /*
        Display Timer
        */

        let time = format!(
            "{}\n{}:{}.{}",
            extra_text,
            clock_time as u32 / 60,
            clock_time as u32 % 60,
            ((clock_time / clock_time.floor() - 1.0) * 1000.0) as u32
        )
        .to_string();
        bus.ppu.tile_maps[timer_map_idx].update_text(time);

        /*
        Event Complete?
        */

        if bl_finished && spy_finished {
            state.events.get_mut(&state.event).unwrap().both_complete = true;
            state.last_event_success = true;
            break 'biathlon;
        }

        /*
        Update game
        */

        // Player
        player.update(bus, &keys, state);

        // catch if fall off bottom of map
        if player.transform.scene_y + 96.0 > bus.ppu.screen_state.map_max_y as f32 {
            state.last_event_success = false;
            break 'biathlon;
        }

        // Game Hot Swap
        if keys.contains(&Keycode::Q) {
            state.swap_game(bus);
            set_tile_attrs(state, false);
        }
        state.swap_cooldown += 1;

        // Camera

        bus.ppu
            .screen_state
            .lerp(next_screen_pos, time_delta * 10.0);

        /*
        Process state
        */

        // video

        let ppu_vals: Vec<u8> = lentsys::ppu::render(
            &bus.ppu.config,
            &bus.ppu.palettes,
            &bus.ppu.tile_sets,
            &bus.ppu.tile_maps,
            &bus.ppu.screen_state,
            &mut bus.ppu.sprites,
        );

        vid.render_frame(ppu_vals, texture);

        // sound

        let audio_data: Vec<f32> = lentsys::apu::render_audio(
            time_delta,
            &mut bus.apu.music,
            &mut bus.apu.synths,
            &mut bus.apu.samples,
            &mut mt,
            &mut bus.apu.fx_queue,
            &bus.apu.config,
        );
        //println!("{:?}", bus.apu.fx_queue.len());
        audio_queue.queue(&audio_data); //&bus.apu.samples[0].data);
        audio_queue.resume();

        //std::thread::sleep(std::time::Duration::from_millis((time_delta * 1000.0) as u64));
    }
    audio_queue.pause();
    audio_queue.clear();
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
            state
            .world
            .collision_set
            .tiles
            .remove_entry(&89);
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
