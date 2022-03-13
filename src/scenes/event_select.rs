use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::TextBox;
use lentsys::game_pak::scene::SceneState;

use crate::game::input::InputCode;
use crate::game::menu::Menu;
use crate::game::state::GameState;

pub fn init(bus: &mut LentSysBus, state: &mut GameState) {
  state.input_cooldown = 15;

  state.menu = Menu {
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

  state.menu.load(bus);

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
    crate::game::state::GameMode::Buglympics => {
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
    crate::game::state::GameMode::Spyder => {}
  }
}

pub fn update(bus: &mut LentSysBus, state: &mut GameState) {
  if state.inputs.contains(&InputCode::Swap) && state.swap_cooldown > 8 {
    state.swap_game(bus);
  }

  state.menu.update_cursor(&state.inputs, bus);

  if state.input_cooldown == 0 && state.menu.confirmed {
    state.event = state.menu.options[state.menu.current_selection].to_string();

    state.current_scene = state.events.get(&state.event).unwrap().scene;

    // set this scene as complete
    bus.game_pak.scenes[state.current_scene].state = SceneState::COMPLETE;

  } else {
    state.menu.confirmed = false;
  }

  if state.input_cooldown > 0 {
    state.input_cooldown -= 1;
  }

  state.swap_cooldown += 1;
}
