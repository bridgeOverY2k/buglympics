use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::TextBox;
use lentsys::game_pak::scene::SceneState;
use crate::game::input::InputCode;
use crate::game::menu::Menu;
use crate::game::state::GameState;

pub fn init(bus: &mut LentSysBus, state: &mut GameState) {
  state.input_cooldown = 15;

  let medals = state.buglympics.medals.get(&state.event).unwrap();

  state.menu = Menu {
    name: String::from("MedalCeremony"),
    screen_x: 0,
    screen_y: 0,
    options: vec![
      String::from(format!("GOLD\n{}", medals.medals[0].time)),
      String::from(format!("SILVER\n{}", medals.medals[1].time)),
      String::from(format!("BRONZE\n{}", medals.medals[2].time)),
    ],
    option_positions: vec![[7 * 16, 12 * 16], [1 * 16, 12 * 16], [15 * 16, 12 * 16]],
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
    input_threshold: 30,
  };

  state.menu.load(bus);

  // Target hit text
  let targets = &state.spyder.events.get(&state.event).unwrap().targets;
  let mut hit_count = 0;
  for tgt in targets.iter() {
    hit_count += tgt.hit as u8;
  }
  let target_result_text = format!("TARGETS HIT {}/{}", hit_count, targets.len());

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
  }
  .to_string();

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
}

pub fn update(bus: &mut LentSysBus, state: &mut GameState) {
  // Hot Swap
  if state.inputs.contains(&InputCode::Swap) && state.swap_cooldown > 8 {
    state.swap_game(bus);
  }

  // Menu inputs
  state.menu.update_cursor(&state.inputs, bus);

  if state.input_cooldown == 0 && state.menu.confirmed {
    // set this scene as complete
    bus.game_pak.scenes[state.current_scene].state = SceneState::COMPLETE;

    state.current_scene = 3;

    bus.game_pak.scenes[state.current_scene].state = SceneState::INITIAL;

  } else {
    state.menu.confirmed = false;
  }

  if state.input_cooldown > 0 {
    state.input_cooldown -= 1;
  }

  state.swap_cooldown += 1;
}
