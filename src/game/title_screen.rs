use std::collections::HashSet;

extern crate sdl2;
use sdl2::audio::AudioQueue;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use zingr::lentsys::LentSysBus;

use crate::game::menu::Menu;
use crate::game::native::NativeVideo;
use crate::game::GameState;

pub fn run_title_screen(
    bus: &mut LentSysBus,
    events: &mut EventPump,
    texture: &mut sdl2::render::Texture,
    vid: &mut NativeVideo,
    audio_queue: &mut AudioQueue<i16>,
    state: &mut GameState,
) {

    let mut title_screen = Menu {
        name: String::from("MainMenu"),
        screen_x: 0,
        screen_y: 0,
        options: vec![String::from("PRESS ENTER")],
        option_positions: vec![[112, 176]],
        current_selection: 0,
        confirmed: false,
        text_tile_set_name: String::from("start_font_small"),
        palette_name: String::from("start_font_small"),
        font_size: 8,
        cursor_tile_set_id: 1,
        cursor_tile_id: 10,
        cursor_sprite_id: 0,
        cursor_offset: [-16, 0],
        input_time: 0,
        input_threshold: 30,
    };

    let mut mt = zingr::apu::music::MusicTracker::new(4);
    let mut last = 0.0;
    let timer = std::time::Instant::now();

    title_screen.load(bus);

    audio_queue.queue(&bus.apu.samples[0].data);
    audio_queue.resume();

    'title_screen: loop {
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

        if keys.contains(&Keycode::Q) {
            state.swap_game(bus);
        }
        state.swap_cooldown += 1;

        title_screen.update_cursor(keys, bus);

        

        if title_screen.confirmed && title_screen.current_selection == 0 {
            break 'title_screen;
        } else {
            title_screen.confirmed = false;
        }

        /*
        Process state
        */

        let ppu_vals: Vec<u8> = zingr::ppu::render(
            &bus.ppu.config,
            &bus.ppu.palettes,
            &bus.ppu.tile_sets,
            &bus.ppu.tile_maps,
            &bus.ppu.screen_state,
            &mut bus.ppu.sprites,
        );

        vid.render_frame(ppu_vals, texture);

        // sound
        let elapsed = timer.elapsed().as_secs_f32();
        let time_delta = elapsed - last;
        let audio_data: Vec<i16> = zingr::apu::render_audio(
            time_delta,
            &mut bus.apu.music,
            &mut bus.apu.synths,
            &mut bus.apu.samples,
            &mut mt,
            &mut bus.apu.fx_queue,
            &bus.apu.config
        );
        //println!("{:?}", bus.apu.fx_queue.len());
        audio_queue.queue(&audio_data);//&bus.apu.samples[0].data);
        audio_queue.resume();
        last = elapsed;
    }

    audio_queue.pause();
    audio_queue.clear();
}
