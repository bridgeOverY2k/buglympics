use lentsys::lentsys::LentSysBus;
use lentsys::game_pak::scene::SceneState;
use lentsys::ppu::{PPUConfig, PPU, render};
use lentsys::apu::{APUConfig, APU, render_audio};
use lentsys::apu::music::AudioSource;
use lentsys::control::PadControl;
use lentsys::game_pak::{GamePak};
use wasm_bindgen::prelude::*;
extern crate console_error_panic_hook;

mod game;
mod scenes;
use crate::game::state::{init_game_state, GameState};

#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
  #[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

#[wasm_bindgen]
pub struct BlSpy {
  bus: LentSysBus,
  image_data: Vec<u8>,
  audio_data: Vec<f32>,
  state: GameState,
}

#[wasm_bindgen]
impl BlSpy {
  
  pub fn new(game_pak_bin: &[u8]) -> Self {
    
      console_error_panic_hook::set_once();
 
      let w: u32 = 320;
      let h: u32 = 240;

      let decoded_game_pak = GamePak::from_bytes(game_pak_bin);

      Self {
        image_data: (0..(w * h * 4)).map(|_| 0).collect(),
        audio_data: vec![],
        bus: LentSysBus {
          controllers: [PadControl::new()],
          apu: APU::new(
             APUConfig {
              sample_rate: 44100.0,
              max_synths: 8,
              max_samples: 8,
             }
          ),
          ppu: PPU::new(PPUConfig {
            res_width: w as u16,
            res_height: h as u16
          }),
          game_pak: decoded_game_pak
        },
        state : init_game_state()
      }
  }

  pub fn update(&mut self) {
    let mut won_all = true;

    // If the current scene is not RUNNING, it is most likely INITIAL or COMPLETE.
    // Load next scene.
    if self.bus.game_pak.scenes[self.state.current_scene].state != SceneState::RUNNING {
      
      // check if game complete
      for (_key, event) in self.state.events.iter(){
        if won_all 
        && (event.scene > 4 && event.scene < 8) {
          // should only run if both are complete
          won_all = event.both_complete;
        }
      }
      
      // it's over!
      if won_all {
        self.state.current_scene = 8;
      }

      self.load_scene();

    } 

    if self.bus.game_pak.scenes[self.state.current_scene].state == SceneState::RUNNING {
      match self.state.current_scene {
        0 => {
          scenes::title_screen::update(&mut self.bus, &mut self.state);
        }
        1 => {
          scenes::attract_mode::update(&mut self.bus, &mut self.state);
        },
        2 => {
          scenes::nation_select::update(&mut self.bus, &mut self.state);
        },
        3 => {
          scenes::event_select::update(&mut self.bus, &mut self.state);
        },
        4 => {
          scenes::medal_cere::update(&mut self.bus, &mut self.state);
        },
        8 => {
          scenes::victory::update(&mut self.bus, &mut self.state);
        },
        5.. => {
          scenes::biathlon::update(&mut self.bus, &mut self.state);
        },
        _ => {
          log("Scene not found");
        }
      }
    }

  }

  fn load_scene(&mut self) {

    // state will be set to LOADING while method is run
    
    self.bus.game_pak.scenes[self.state.current_scene].load(
      &mut self.bus.ppu,
      &mut self.bus.apu,
      &mut self.bus.game_pak.assets,
    );

    // scene state should now be set to RUNNING
    match self.state.current_scene {
      0 => {
        scenes::title_screen::init(&mut self.bus, &mut self.state);
      },
      1 => {
        scenes::attract_mode::init(&mut self.bus, &mut self.state);
      },
      2 => {
        scenes::nation_select::init(&mut self.bus, &mut self.state);
      },
      3 => {
        scenes::event_select::init(&mut self.bus, &mut self.state);
      },
      4 => {
        scenes::medal_cere::init(&mut self.bus, &mut self.state);
      },
      8 => {
        scenes::victory::init(&mut self.bus, &mut self.state);
      }
      5.. => {
        scenes::biathlon::init(&mut self.bus, &mut self.state);
      },
      _ => {

      }
    }

  }

  pub fn get_image_data(&self) -> *const u8 {
    self.image_data.as_ptr()
  }
  
  pub fn render_image(&mut self){

    self.image_data = render(
      &self.bus.ppu.config,
      &self.bus.ppu.palettes,
      &self.bus.ppu.tile_sets,
      &self.bus.ppu.tile_maps,
      &self.bus.ppu.screen_state,
      &mut self.bus.ppu.sprites,
    );
  
  }

  pub fn get_audio_data(&self) -> *const f32 {
    self.audio_data.as_ptr()
  }

  pub fn render_audio(&mut self, time_delta: f32){
    let samples = self.bus.apu.config.sample_rate * time_delta;

    // bg music sample
    if self.bus.apu.samples.len() > 0 {
      self.state.sfx_queue.push(
        (0.0, AudioSource::Sample, 0, samples as usize)
      );
    }
    
    self.audio_data = render_audio(
      time_delta,
      &mut self.bus.apu.music,
      &mut self.bus.apu.synths,
      &mut self.bus.apu.samples,
      &mut self.state.music_tracker,
      &mut self.state.sfx_queue,
      &self.bus.apu.config
    );
  }

  pub fn set_inputs(&mut self, controller: &PadControl){
    self.bus.controllers[0] = *controller;
    self.state.inputs = game::input::map_input(self.bus.controllers[0]);
  }
}

pub trait Native {
  fn get_image(&self) -> &Vec<u8>;
  fn get_audio(&self) -> &Vec<f32>;
}

impl Native for BlSpy {
  fn get_image(&self) -> &Vec<u8> {
    &self.image_data
  }

  fn get_audio(&self) -> &Vec<f32> {
    &self.audio_data
  }
}