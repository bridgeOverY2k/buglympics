use lentsys::lentsys::LentSysBus;
use lentsys::game_pak::scene::SceneState;
use crate::game::input::InputCode;
use crate::game::menu::Menu;
use crate::game::state::{GameState};

pub fn init(bus: &mut LentSysBus, state: &mut GameState) {
    state.menu = Menu {
        name: String::from("MainMenu"),
        screen_x: 0,
        screen_y: 0,
        options: vec![String::from("PRESS ENTER")],
        option_positions: vec![[112, 176]],
        current_selection: 0,
        confirmed: false,
        text_tile_set_name: String::from("start_font_small"),
        palette_name: String::from("start_font_small"),
        font_size: 8,
        cursor_tile_set_id: 1,
        cursor_tile_id: 10,
        cursor_sprite_id: 0,
        cursor_offset: [-16, 0],
        input_time: 0,
        input_threshold: 30,
    };

    state.menu.load(bus);
}

pub fn update(bus: &mut LentSysBus, state: &mut GameState){

    if state.inputs.contains(&InputCode::Swap) {

        state.swap_game(bus);
    
    }

    if state.inputs.contains(&InputCode::Confirm) {
        
        state.menu.confirmed = true;
        
        // set this scene as complete
        bus.game_pak.scenes[state.current_scene].state = SceneState::COMPLETE;

        // set next scene
        state.current_scene = 1;

    }

    state.swap_cooldown += 1;

}
