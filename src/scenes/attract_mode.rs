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

pub fn run_attract_mode(
  bus: &mut LentSysBus,
  events: &mut EventPump,
  texture: &mut sdl2::render::Texture,
  vid: &mut NativeVideo,
  audio_queue: &mut AudioQueue<f32>,
  state: &mut GameState,
) {
  let _timer = Instant::now();
  let mut current_shot = 0;
  let mut input_cooldown = 15;
  let buglympics_shots = vec![
    Shot {
      tile_map_name: String::from("cut_01_arrival"),
      tile_set_name: String::from("buglympics_shot_01"),
      duration: 0,
      text: TextBox::new(
        String::from(
          "Once again, the bug nations of\n\
            Earthropod have set aside \
            their differences to compete in the\n\
            ancient spectacle known as\n\
            The Winter Buglympics.",
        ),
        32.0,
        160.0,
        String::from("start_font_small"),
        String::from("start_font_small"),
        8,
        Some(32),
        Some(6),
      ),
    },
    Shot {
      tile_map_name: String::from("cut_01_arrival"),
      tile_set_name: String::from("buglympics_shot_02"),
      duration: 0,
      text: TextBox::new(
        String::from(
          "Entrants -- nematode, arachnid, \
            and insect alike -- have arrived \
            in the capital city of\n\
            Tarantulum in East Archanylvania",
        ),
        32.0,
        160.0,
        String::from("start_font_small"),
        String::from("start_font_small"),
        8,
        Some(32),
        Some(6),
      ),
    },
    Shot {
      tile_map_name: String::from("cut_02_park"),
      tile_set_name: String::from("cut_02_park"),
      duration: 0,
      text: TextBox::new(
        String::from(
          "Sportsbugship and diplomacy     \
            steam in the Webruary air.      \
            Are you ready?",
        ),
        32.0,
        160.0,
        String::from("start_font_small"),
        String::from("start_font_small"),
        8,
        Some(32),
        Some(6),
      ),
    },
  ];

  let spyder_shots = vec![
    Shot {
      tile_map_name: String::from("cut_01_arrival"),
      tile_set_name: String::from("spyder_shot_01"),
      duration: 0,
      text: TextBox::new(
        String::from(
          "It's the Arachnyvanian \n\
          Chancellor, Antsly. She's hidden \
          listening devices throughout     \
          The Buglympics Village.          \
          You must destroy them.",
        ),
        32.0,
        160.0,
        String::from("start_font_small"),
        String::from("title_screen_hh"),
        8,
        Some(32),
        Some(6),
      ),
    },
    Shot {
      tile_map_name: String::from("cut_01_arrival"),
      tile_set_name: String::from("spyder_shot_02"),
      duration: 0,
      text: TextBox::new(
        String::from(
          "They nearly caught my phony \
          passport. Maybe they're expecting me? \
          I better stay on my toes...",
        ),
        32.0,
        160.0,
        String::from("start_font_small"),
        String::from("title_screen_hh"),
        8,
        Some(32),
        Some(6),
      ),
    },
    Shot {
      tile_map_name: String::from("cut_01_arrival"),
      tile_set_name: String::from("spyder_shot_03"),
      duration: 0,
      text: TextBox::new(
        String::from(
          "Hmm...Nothing in the village. \n\
          I'll sneak on the slopes tonight when it gets dark...",
        ),
        32.0,
        160.0,
        String::from("start_font_small"),
        String::from("title_screen_hh"),
        8,
        Some(32),
        Some(6),
      ),
    },
  ];

  audio_queue.queue(&bus.apu.samples[1].play());
  audio_queue.resume();
  match &state.game {
    GameMode::Buglympics => {
      buglympics_shots[current_shot].load(bus);
    }
    GameMode::Spyder => {
      spyder_shots[current_shot].load(bus);
    }
  }

  'attract_mode: loop {
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
      current_shot += 1;
      input_cooldown = 15;
      if current_shot > buglympics_shots.len() - 1 {
        break 'attract_mode;
      }
      match &state.game {
        GameMode::Buglympics => {
          buglympics_shots[current_shot].load(bus);
        }
        GameMode::Spyder => {
          spyder_shots[current_shot].load(bus);
        }
      }
    }

    if keys.contains(&Keycode::Q) && input_cooldown == 0 {
      state.swap_game(bus);
      match &state.game {
        GameMode::Buglympics => {
          buglympics_shots[current_shot].load(bus);
        }
        GameMode::Spyder => {
          spyder_shots[current_shot].load(bus);
        }
      }
    }

    state.swap_cooldown += 1;

    if input_cooldown > 0 {
      input_cooldown -= 1;
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
  }
  audio_queue.pause();
  audio_queue.clear();
}
