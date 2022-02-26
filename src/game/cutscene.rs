use std::collections::HashSet;
use crate::game::menu::Menu;
use std::collections::HashMap;
use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::TextBox;

use sdl2::keyboard::Keycode;

pub struct Shot {
  pub tile_map_name: String,
  pub tile_set_name: String,
  pub text: TextBox,
  pub duration: u64,
}

impl Shot {
  pub fn load(&self, bus: &mut LentSysBus) {
    
    bus.ppu.flush();

    // tile_set and palette
    let (ts, pal) = bus.game_pak.assets.get_tile_set(self.tile_set_name.to_string());
    bus.ppu.palettes.push(pal);
    bus.ppu.tile_sets.push(ts);
    
    // image tile_map
    let mut tms = bus.game_pak.assets.get_tile_map(self.tile_map_name.to_string());
    for tm in tms.iter_mut() {
      tm.tile_set_id = bus.ppu.tile_sets.len() - 1;
      tm.palette_id = bus.ppu.palettes.len() - 1;
    }
    bus.ppu.tile_maps.append(&mut tms);

    // text tile_map
    self.text.to_tilemap(bus);    
    
  }
}

pub struct Prompt {
  pub displayed: bool,
  pub menu: Menu,
  pub next_shot: usize
}

pub struct CutScene {
  pub name: String,
  pub timer: u64,
  pub shot_timer: u64,
  pub shots: Vec<Shot>,
  pub current_shot: usize,
  pub prompts: HashMap<String, Prompt>,
  pub current_prompt: String,
}

impl CutScene {
  pub fn start(&mut self, bus: &mut LentSysBus) {
    self.timer = 0;
    self.load_shot(bus);
    self.load_prompt(bus);
  }

  pub fn load_shot(&mut self, bus: &mut LentSysBus) {
    let shot = &self.shots[self.current_shot];
    shot.load(bus);
  }

  pub fn load_prompt(&mut self, bus: &mut LentSysBus){
    let prompt = self.prompts.get_mut(&self.current_prompt).unwrap();
    prompt.menu.load(bus);
  }

  pub fn update(&mut self, keys: HashSet<Keycode>, bus: &mut LentSysBus) {
    let shot_duration = self.shots[self.current_shot].duration;
    let prompt = self.prompts.get_mut(&self.current_prompt).unwrap();
    if shot_duration != 0 && self.shot_timer > shot_duration {
      if self.current_shot < self.shots.len() - 1 {
        self.current_shot += 1;
        self.load_shot(bus);
        self.load_prompt(bus);
        self.shot_timer = 0;
      }
    } else {
      prompt.menu.update_cursor(keys, bus);
      self.shot_timer += 1;
    }
    self.timer += 1;
  }

}

mod test {
  use super::*;

  fn _test_cutscene(){

    let _cut_scene = CutScene {
      name: String::from("Attract Story"),
      timer: 0,
      shot_timer: 0,
      shots: vec![
        Shot {
          tile_map_name: String::from("cut_02_park"),
          tile_set_name: String::from("cut_02_park"),
          duration: 600,
          text: TextBox::new(
              String::from("Once again, the bug nations of\n\
              Earthropod have set aside \
              their differences to compete in the\n\
              ancient spectacle known as\n\
              The Winter Buglympics."),
              32.0, 
              160.0,
              String::from("start_font_small"),
              String::from("start_font_small"),
              8,
              Some(32),
              Some(6),
            )
        },
        Shot {
          tile_map_name: String::from("cut_01_arrival"),
          tile_set_name: String::from("cut_01_arrival"),
          duration: 540,
          text: TextBox::new(
              String::from("Entrants -- nematode, arachnid, \
              and insect alike -- have arrived from \
              the four corners of the globe to the \
              capital city of\n\
              Tarantulum in East Archanylvania."),
              32.0, 
              160.0,
              String::from("start_font_small"),
              String::from("start_font_small"),
              8,
              Some(32),
              Some(6),
            )
        },
        Shot {
          tile_map_name: String::from("attract_bg_01"),
          tile_set_name: String::from("winter_set"),
          duration: 540,
          text: TextBox::new(
              String::from("Sportsbugship, diplomacy, \
              and cunning steam in the \
              crisp Webuary air."),
              64.0, 
              128.0,
              String::from("start_font_small"),
              String::from("start_font_small"),
              16,
              Some(32),
              Some(6),
            )
        }],
      current_shot: 0,
      prompts: vec![(
        String::from("next"),
        Prompt {
        displayed: false,
        next_shot: 1,
        menu: Menu {
          name: String::from("next"),
          screen_x: 0,
          screen_y: 0,
          options: vec![String::from(">")],
          option_positions: vec![[280, 216]],
          current_selection: 0,
          confirmed: false,
          text_tile_set_name: String::from("start_font"),
          palette_name: String::from("start_font"),
          font_size: 16,
          cursor_tile_set_id: 2,
          cursor_tile_id: 46,
          cursor_sprite_id: 0,
          cursor_offset: [-8, 0],
          input_time: 0,
          input_threshold: 30
        }
      })].into_iter().collect(),
      current_prompt: String::from("next"),
    };

  }

}
