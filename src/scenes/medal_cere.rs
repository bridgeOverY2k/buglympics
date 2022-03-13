use std::collections::HashSet;
use std::time::Instant;

extern crate sdl2;
use sdl2::keyboard::Keycode;
use sdl2::audio::AudioQueue;
use sdl2::EventPump;
use sdl2::event::Event;

use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::TextBox;

use crate::game::menu::Menu;
use crate::native::NativeVideo;
use crate::game::state::GameState;

pub fn run_medal_cere(
  bus: &mut LentSysBus,
  events: &mut EventPump,
  texture: &mut sdl2::render::Texture,
  vid: &mut NativeVideo,
  _audio_queue: &mut AudioQueue<f32>,
  state: &mut GameState,
) {

  let timer = Instant::now();

  let medals = state.buglympics.medals.get(&state.event).unwrap();

  let mut medal_listing = Menu {
      name: String::from("MedalCeremony"),
      screen_x: 0,
      screen_y: 0,
      options: vec![
        String::from(format!("GOLD\n{}", medals.medals[0].time)),
        String::from(format!("SILVER\n{}", medals.medals[1].time)),
        String::from(format!("BRONZE\n{}", medals.medals[2].time)),
      ],
      option_positions: vec![
        [7 * 16, 12 * 16],
        [1 * 16, 12 * 16],
        [15 * 16, 12 * 16]
      ],
      current_selection: 0,
      confirmed: false,
      text_tile_set_name: String::from("start_font_small"),
      palette_name: String::from("start_font_small"),
      font_size: 8,
      cursor_tile_set_id: 2,
      cursor_tile_id: 9,
      cursor_sprite_id: 0,
      cursor_offset: [-12, 0],
      input_time: 0,
      input_threshold: 30
  };

  medal_listing.load(bus);

  // Targets 
  let targets = &state.spyder.events.get(&state.event).unwrap().targets;
  let mut hit_count = 0;
  for tgt in targets.iter(){
    hit_count += tgt.hit as u8;
  }
  let target_result_text = format!(
    "TARGETS HIT {}/{}",
    hit_count, targets.len()
  );

  let target_textbox = TextBox::new(
    target_result_text,
    16.0 * 6.0,
    16.0 * 13.0 + 8.0,
    String::from("start_font_small"),
    String::from("title_screen_hh"),
    8,
    Some(32),
    Some(1),
  );

  target_textbox.to_tilemap(bus);
  let mut last_tm = bus.ppu.tile_maps.len() - 1;
  bus.ppu.tile_maps[last_tm].order = 1;

  // Main Text Banner
  let banner_text: String = if state.last_event_success {
    "RESULTS"
  } else {
    "FAILED"
  }.to_string();

  let banner_textbox = TextBox::new(
    banner_text,
    16.0 * 6.0,
    16.0 * 11.0,
    String::from("start_font"),
    String::from("start_font"),
    16,
    Some(20),
    Some(1),
  );

  banner_textbox.to_tilemap(bus);
  last_tm = bus.ppu.tile_maps.len() - 1;
  bus.ppu.tile_maps[last_tm].order = 1;

  // Check which game we're on
  state.event = String::from("medal_cere");
  state.check_game(bus);

  'medals: loop {
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

      // Hot Swap
      if keys.contains(&Keycode::Q) && state.swap_cooldown > 8 {
        state.swap_game(bus);
      }
      state.swap_cooldown += 1;

      // Menu inputs
      medal_listing.update_cursor(keys, bus);

      if timer.elapsed().as_secs_f32() > 1.5
      && medal_listing.confirmed {
          break 'medals;
      } else {
        medal_listing.confirmed = false;
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

      vid.render_frame(
          ppu_vals,
          texture,
      );
  }
}