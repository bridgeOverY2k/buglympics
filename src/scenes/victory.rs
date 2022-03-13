use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::TextBox;

use crate::game::cutscene::Shot;
use crate::game::input::InputCode;
use crate::game::state::GameMode;
use crate::game::state::GameState;

pub fn init(bus: &mut LentSysBus, state: &mut GameState) {
  state.input_cooldown = 15;
  state.bl_shots = vec![Shot {
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
  }];

  state.spyder_shots = vec![Shot {
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
  }];

  state.current_shot = 0;

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

  state.swap_cooldown += 1;

  if state.input_cooldown > 0 {
    state.input_cooldown -= 1;
  }
}
