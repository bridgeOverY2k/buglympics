use std::collections::HashSet;
use std::time::Instant;

extern crate sdl2;
use sdl2::audio::AudioQueue;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::TextBox;

use crate::game::menu::Menu;
use crate::game::native::NativeVideo;
use crate::game::GameState;

pub fn run_event_select(
  bus: &mut LentSysBus,
  events: &mut EventPump,
  texture: &mut sdl2::render::Texture,
  vid: &mut NativeVideo,
  audio_queue: &mut AudioQueue<f32>,
  state: &mut GameState,
) {
  let timer = Instant::now();
  let mut mt = lentsys::apu::music::MusicTracker::new(4);
  let mut last = 0.0;

  let mut event_select = Menu {
    name: String::from("Event Select"),
    screen_x: 0,
    screen_y: 0,
    options: vec![
      String::from("CROSS-COUNTRY BIATHLON"),
      String::from("DOWNHILL BIATHLON"),
      String::from("CRAGGY BIATHLON"),
    ],
    option_positions: vec![[48, 64], [48, 128], [48, 192]],
    current_selection: 0,
    confirmed: false,
    text_tile_set_name: String::from("start_font_small"),
    palette_name: String::from("start_font_small"),
    font_size: 8,
    cursor_tile_set_id: 1,
    cursor_tile_id: 9,
    cursor_sprite_id: 0,
    cursor_offset: [-12, 0],
    input_time: 0,
    input_threshold: 5,
  };

  event_select.load(bus);

  let instruct = TextBox::new(
    String::from("SELECT YOUR EVENT"),
    16.0,
    16.0,
    String::from("start_font"),
    String::from("start_font"),
    16,
    Some(20),
    Some(1),
  );

  instruct.to_tilemap(bus);
  let last_tm = bus.ppu.tile_maps.len() - 1;
  bus.ppu.tile_maps[last_tm].order = 1;

  state.check_game(bus);

  match state.game {
    crate::GameMode::Buglympics => {
      let medal_places = ["GOLD", "SILVER", "BRONZE"];
      for (key, medal_standing) in state.buglympics.medals.iter() {
        for (place, medal) in medal_standing.medals.iter().enumerate() {
          if medal.nation == state.buglympics.nation {
            let x = 160.0;
            let y = match key.as_str() {
              "CROSS-COUNTRY BIATHLON" => 80.0,
              "DOWNHILL BIATHLON" => 144.0,
              "CRAGGY BIATHLON" => 204.0,
              _ => 0.0,
            };

            TextBox::new(
              format!("{0} - {1:.2}", medal_places[place], medal.time).to_string(),
              x,
              y,
              String::from("start_font_small"),
              String::from("start_font_small"),
              8,
              Some(16),
              Some(1),
            )
            .to_tilemap(bus);

            let last_idx = bus.ppu.tile_maps.len() - 1;
            bus.ppu.tile_maps[last_idx].order = 1;
          }
        }
      }
    }
    crate::GameMode::Spyder => {}
  }

  'event_select: loop {
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

    if keys.contains(&Keycode::Q) && state.swap_cooldown > 8 {
      state.swap_game(bus);
    }
    state.swap_cooldown += 1;

    event_select.update_cursor(keys, bus);

    if timer.elapsed().as_secs_f32() > 1.5 && event_select.confirmed {
      state.event = event_select.options[event_select.current_selection].to_string();
      break 'event_select;
    } else {
      event_select.confirmed = false;
    }

    /*
    Process state
    */

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
    let elapsed = timer.elapsed().as_secs_f32();
    let time_delta = elapsed - last;
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
    last = elapsed;
  }
  audio_queue.pause();
  audio_queue.clear();
}
