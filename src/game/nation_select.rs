use std::collections::HashSet;
use std::time::Instant;

extern crate sdl2;
use sdl2::audio::AudioQueue;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use zingr::lentsys::LentSysBus;
use zingr::ppu::text::Text;
use zingr::ppu::text::TextBox;

use crate::game::menu::Menu;
use crate::game::native::NativeVideo;
use crate::game::{GameMode, GameState};

pub fn run_nation_select(
  bus: &mut LentSysBus,
  events: &mut EventPump,
  texture: &mut sdl2::render::Texture,
  vid: &mut NativeVideo,
  audio_queue: &mut AudioQueue<i16>,
  state: &mut GameState,
) {
  let timer = Instant::now();
  let mut mt = zingr::apu::music::MusicTracker::new(4);
  let mut last = 0.0;

  let mut nation_select = Menu {
    name: String::from("NationSelect"),
    screen_x: 0,
    screen_y: 0,
    options: vec![
      String::from("ANTARTICA"),
      String::from("EAST ARACHNYLVANIA"),
      String::from("REP. OF WORMSTRALIA"),
    ],
    option_positions: vec![[16, 64], [16, 128], [16, 192]],
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

  nation_select.load(bus);

  let instruct = TextBox::new(
    String::from("SELECT YOUR NATION"),
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
  swap_text(&state.game, bus);

  //Music
  audio_queue.queue(&bus.apu.samples[2].data);
  audio_queue.resume();

  'nation_select: loop {
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

      swap_text(&state.game, bus);
    }

    nation_select.update_cursor(keys, bus);

    state.swap_cooldown += 1;

    if timer.elapsed().as_secs_f32() > 1.5 && nation_select.confirmed {
      state.buglympics.nation = nation_select.options[nation_select.current_selection].to_string();
      break 'nation_select;
    } else {
      nation_select.confirmed = false;
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
      &bus.apu.config,
    );
    //println!("{:?}", bus.apu.fx_queue.len());
    audio_queue.queue(&audio_data); //&bus.apu.samples[0].data);
    audio_queue.resume();
    last = elapsed;
  }
}

pub fn swap_text(game: &GameMode, bus: &mut LentSysBus) {
  let last_tm = bus.ppu.tile_maps.len() - 1;
  match game {
    GameMode::Buglympics => {
      bus.ppu.tile_maps[last_tm].update_text(String::from("SELECT YOUR NATION"));
      bus.ppu.tile_maps[last_tm - 3].update_text(String::from("ANTARTICA"));
      bus.ppu.tile_maps[last_tm - 2].update_text(String::from("EAST ARACHNYLVANIA"));
      bus.ppu.tile_maps[last_tm - 1].update_text(String::from("REP. OF WORMSTRALIA"));
      bus.ppu.tile_maps[last_tm].palette_id = 2;
      bus.ppu.tile_maps[last_tm - 1].palette_id = 2;
      bus.ppu.tile_maps[last_tm - 2].palette_id = 2;
      bus.ppu.tile_maps[last_tm - 3].palette_id = 2;
    }
    GameMode::Spyder => {
      bus.ppu.tile_maps[last_tm].update_text(String::from("SELECT YOUR TOOL"));
      bus.ppu.tile_maps[last_tm - 3].update_text(String::from("SILENT AX"));
      bus.ppu.tile_maps[last_tm - 2].update_text(String::from("QUIET BLUDGEON"));
      bus.ppu.tile_maps[last_tm - 1].update_text(String::from("THE CRAB"));
      bus.ppu.tile_maps[last_tm].palette_id = 1;
      bus.ppu.tile_maps[last_tm - 1].palette_id = 1;
      bus.ppu.tile_maps[last_tm - 2].palette_id = 1;
      bus.ppu.tile_maps[last_tm - 3].palette_id = 1;
    }
  }
}
