use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::TextBox;
use lentsys::game_pak::scene::SceneState;
use crate::game::cutscene::Shot;
use crate::game::input::InputCode;
use crate::game::state::GameMode;
use crate::game::state::GameState;

pub fn init(bus: &mut LentSysBus, state: &mut GameState) {
  state.current_shot = 0;
  state.input_cooldown = 15;
  state.bl_shots = vec![
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

  state.spyder_shots = vec![
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

  match &state.game {
    GameMode::Buglympics => {
      state.bl_shots[state.current_shot].load(bus);
    }
    GameMode::Spyder => {
      state.spyder_shots[state.current_shot].load(bus);
    }
  }
}

pub fn update(bus: &mut LentSysBus, state: &mut GameState) {

  // next shot, please
  if state.inputs.contains(&InputCode::Confirm) && state.input_cooldown == 0 {
    state.current_shot += 1;
    state.input_cooldown = 15;
    // if this was the last shot, the scene is over
    // the length of both are the same.
    if state.current_shot > state.bl_shots.len() - 1 {

      // set this scene as complete
      bus.game_pak.scenes[state.current_scene].state = SceneState::COMPLETE;
      
      state.current_scene = 2;
    
    } else {
      match &state.game {
        GameMode::Buglympics => {
          state.bl_shots[state.current_shot].load(bus);
        }
        GameMode::Spyder => {
          state.spyder_shots[state.current_shot].load(bus);
        }
      }
    }
  }

  //swap game
  if state.inputs.contains(&InputCode::Swap) && state.input_cooldown == 0 {
    state.swap_game(bus);
    match &state.game {
      GameMode::Buglympics => {
        state.bl_shots[state.current_shot].load(bus);
      }
      GameMode::Spyder => {
        state.spyder_shots[state.current_shot].load(bus);
      }
    }
  }

  if state.input_cooldown > 0 {
    state.input_cooldown -= 1;
  }

  state.swap_cooldown += 1;
}
