use std::collections::HashSet;

use lentsys::lentsys::LentSysBus;
use lentsys::ppu::sprite::Sprite;
use lentsys::ui::text::TextBox;

use crate::game::sounds;
use crate::game::input::{InputCode};

pub struct Menu {
  pub name: String,
  pub screen_x: u16,
  pub screen_y: u16,
  pub options: Vec<String>,
  pub option_positions: Vec<[u16; 2]>,
  pub current_selection: usize,
  pub confirmed: bool,
  pub text_tile_set_name: String,
  pub palette_name: String,
  pub font_size: u16,
  pub cursor_tile_set_id: usize,
  pub cursor_tile_id: usize,
  pub cursor_sprite_id: usize,
  pub cursor_offset: [i16; 2],
  pub input_time: u16,
  pub input_threshold: u16,
}

impl Menu {
  pub fn load(&mut self, bus: &mut LentSysBus) {
    self.input_time = 0;
    self.input_threshold = 3;
    for (idx, opt) in self.options.iter().enumerate() {
      TextBox::new(
        String::from(opt),
        self.option_positions[idx][0] as f32,
        self.option_positions[idx][1] as f32,
        self.text_tile_set_name.to_string(),
        self.palette_name.to_string(),
        self.font_size,
        None,
        None,
      )
      .to_tilemap(bus);

      // TODO: actually handle order
      let last_tm = bus.ppu.tile_maps.len() - 1;
      bus.ppu.tile_maps[last_tm].order = 1;
    }

    bus.ppu.sprites.push(Sprite {
      entity_id: 0,
      tile_set_id: self.cursor_tile_set_id,
      tile_id: self.cursor_tile_id as u16,
      palette_id: (bus.ppu.palettes.len() - 1) as u16,
      lines_drawn: 0,
      scene_x: (self.option_positions[self.current_selection][0] as i16 + self.cursor_offset[0])
        as u16,
      scene_y: (self.option_positions[self.current_selection][1] as i16 + self.cursor_offset[1])
        as u16,
      reverse_x: false,
      reverse_y: false,
      width: self.font_size,
      height: self.font_size,
      hide: false,
      expired: false,
    });

    self.cursor_sprite_id = bus.ppu.sprites.len() - 1;

    sounds::prepare_effects(bus);
  }

  pub fn update_cursor(&mut self, keys: HashSet<InputCode>, bus: &mut LentSysBus) {
    let mut any_input = false;
    if keys.contains(&InputCode::Down) || keys.contains(&InputCode::Right) {
      self.input_time += 1;
      if self.input_time > self.input_threshold && self.options.len() - 1 > 0 {
        self.current_selection += 1;
        sounds::play_effect(bus, sounds::SFX::Switch, 800);
        self.input_time = 0;
      }
      
      any_input = true;
    }

    if keys.contains(&InputCode::Up) || keys.contains(&InputCode::Left) {
      self.input_time += 1;
      if self.input_time > self.input_threshold && self.options.len() - 1 > 0 {
        if self.current_selection == 0 {
          self.current_selection = self.options.len() - 1;
        } else {
          self.current_selection -= 1;
        }
        sounds::play_effect(bus, sounds::SFX::Switch, 800);
        self.input_time = 0;
      }
      any_input = true;
    }

    if keys.contains(&InputCode::Fire) || keys.contains(&InputCode::Confirm) {
      self.confirmed = true;
      any_input = true;
      // sound
      sounds::play_effect(bus, sounds::SFX::Select, 800);
    }

    if self.current_selection >= self.options.len() {
      self.current_selection = 0;
    }

    if !any_input {
      self.input_time = 0;
    }

    bus.ppu.sprites[self.cursor_sprite_id].scene_x =
      (self.option_positions[self.current_selection][0] as i16 + self.cursor_offset[0]) as u16;
    bus.ppu.sprites[self.cursor_sprite_id].scene_y =
      (self.option_positions[self.current_selection][1] as i16 + self.cursor_offset[1]) as u16;
  }
}

mod test {
  use super::*;

  fn _test_menu() {
    let _title_screen = Menu {
      name: String::from("MainMenu"),
      screen_x: 0,
      screen_y: 0,
      options: vec![String::from("Start"), String::from("Options")],
      option_positions: vec![[128, 128], [128, 160]],
      current_selection: 0,
      confirmed: false,
      text_tile_set_name: String::from("tileset_name"),
      palette_name: String::from("palette_name"),
      font_size: 16,
      cursor_tile_set_id: 0,
      cursor_tile_id: 16,
      cursor_sprite_id: 0,
      cursor_offset: [-16, 0],
      input_time: 0,
      input_threshold: 30,
    };
  }
}
