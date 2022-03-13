use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::{TextBox, Text};
use lentsys::game_pak::scene::SceneState;
use crate::game::input::InputCode;
use crate::game::menu::Menu;
use crate::game::state::{GameMode, GameState};

pub fn init(bus: &mut LentSysBus, state: &mut GameState) {
  state.input_cooldown = 15;
  state.menu = Menu {
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

  state.menu.load(bus);

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
}

pub fn update(bus: &mut LentSysBus, state: &mut GameState) {
  
  if state.inputs.contains(&InputCode::Swap) && state.swap_cooldown > 8 {
    state.swap_game(bus);
    swap_text(&state.game, bus);
  }

  state.menu.update_cursor(&state.inputs, bus);

  if state.input_cooldown == 0 && state.menu.confirmed {
    
    state.buglympics.nation = state.menu.options[state.menu.current_selection].to_string();

    // set this scene as complete
    bus.game_pak.scenes[state.current_scene].state = SceneState::COMPLETE;

    state.current_scene = 3;
  
  } else {
    state.menu.confirmed = false;
  }

  if state.input_cooldown > 0 {
    state.input_cooldown -= 1;
  }

  state.swap_cooldown += 1;
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
