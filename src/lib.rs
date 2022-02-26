use lentsys::web::LentSysWeb;
mod game;
use crate::game::state::{init_game_state, GameState};

pub struct BlSpy {
  pub lentsys_web: LentSysWeb,
  state: GameState
}

impl BlSpy {
  pub fn start(&mut self){
    self.state = init_game_state();
  }
}