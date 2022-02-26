use std::collections::HashSet;

extern crate sdl2;
use sdl2::audio::AudioQueue;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::TextBox;

use std::time::Instant;

use crate::game::cutscene::Shot;
use crate::game::state::GameMode;
use crate::game::state::GameState;

use crate::game::native::NativeVideo;

pub fn run_victory(
  bus: &mut LentSysBus,
  events: &mut EventPump,
  texture: &mut sdl2::render::Texture,
  vid: &mut NativeVideo,
  audio_queue: &mut AudioQueue<f32>,
  state: &mut GameState,
) {
  let _timer = Instant::now();
  let mut input_cooldown = 15;
  let buglympics_shot = Shot {
    tile_map_name: String::from("cut_01_arrival"),
    tile_set_name: String::from("buglympics_victory"),
    duration: 0,
    text: TextBox::new(
      String::from(
        "That's a new world record! \n\
          \n\n\n\
          THE END",
      ),
      32.0,
      160.0,
      String::from("start_font_small"),
      String::from("start_font_small"),
      8,
      Some(32),
      Some(6),
    ),
  };

  let spyder_shot = Shot {
    tile_map_name: String::from("cut_01_arrival"),
    tile_set_name: String::from("spyder_victory"),
    duration: 0,
    text: TextBox::new(
      String::from(
        "I found the listening devices! \n\
          Operation Spyder was a success. \n\
          MISSION ACCOMPLISHED",
      ),
      32.0,
      160.0,
      String::from("start_font_small"),
      String::from("start_font_small"),
      8,
      Some(32),
      Some(6),
    ),
  };

  match &state.game {
    GameMode::Buglympics => {
      buglympics_shot.load(bus);
    }
    GameMode::Spyder => {
      spyder_shot.load(bus);
    }
  }

  audio_queue.queue(&bus.apu.samples[1].play());
  audio_queue.resume();

  'victory: loop {
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

    if keys.contains(&Keycode::Z) && input_cooldown == 0 {
      break 'victory;
    }

    if keys.contains(&Keycode::Q) && input_cooldown == 0 {
      state.swap_game(bus);
      match &state.game {
        GameMode::Buglympics => {
          buglympics_shot.load(bus);
        }
        GameMode::Spyder => {
          spyder_shot.load(bus);
        }
      }
    }

    state.swap_cooldown += 1;

    if input_cooldown > 0 {
      input_cooldown -= 1;
    }
    //println!("input cooldown {}", input_cooldown);

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
  }

  audio_queue.pause();
  audio_queue.clear();
}
