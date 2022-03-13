use lentsys::apu::APUConfig;
use lentsys::game_pak::GamePak;
use lentsys::lentsys::LentSys;
use lentsys::lentsys::LentSysBus;
use lentsys::ppu::PPUConfig;

extern crate sdl2;
//use sdl2::audio::AudioQueue;
use sdl2::audio::AudioSpecDesired;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;
use sdl2::render::TextureAccess;

pub mod scenes;
use scenes::attract_mode::run_attract_mode;
use scenes::biathlon::run_biathlon;
use scenes::event_select::run_event_select;
use scenes::title_screen::run_title_screen;
use scenes::medal_cere::run_medal_cere;
use scenes::nation_select::run_nation_select;
use scenes::victory::run_victory;

pub mod native;
use native::NativeVideo;

pub mod game;
use game::*;

fn main() {
    
    let mut lent_sys = LentSys::new(
        APUConfig {
            max_samples: 64,
            max_synths: 8,
            sample_rate: 44100.0,
        },
        PPUConfig {
            res_height: 240,
            res_width: 320,
        },
    );

    lent_sys.bus.game_pak =
        GamePak::from_binary(&String::from("./buglympics.bin"));

    lent_sys.boot(native_game);
}

fn native_game(bus: &mut LentSysBus) -> Result<(), String> {
    // SDL init stuff
    let sdl_context = sdl2::init()?;
    let mut events = sdl_context.event_pump().unwrap();

    let mut vid = NativeVideo::new(
        &sdl_context,
        String::from("Winter Buglympics / SPYDER"),
        bus.ppu.config.res_width as u32,
        bus.ppu.config.res_height as u32,
    );

    let texture_creator = vid.canvas.texture_creator();
    let mut texture = {
        let tex = texture_creator
            .create_texture(
                PixelFormatEnum::RGBA32,
                TextureAccess::Streaming,
                vid.width,
                vid.height,
            )
            .unwrap();
        unsafe { Box::<Texture>::new(std::mem::transmute(tex)) }
    };
    let sample_rate = 44100.0;
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(sample_rate as i32),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };
    let mut audio_queue = audio_subsystem.open_queue::<f32, _>(None, &desired_spec)?;
    
    // *** Start of LentSys stuff ***

    let mut state = game::state::init_game_state();

    // allow game restart
    let mut repeat = false;

    'main: loop {

        let mut won_all = true;
        for (_key, event) in state.events.iter(){
            if won_all 
            && (event.scene > 4 && event.scene < 8) {
                //println!("{} {}", key, event.both_complete);
                won_all = event.both_complete;
            }
        }

        if won_all && !repeat {
            state.current_scene = 8;
        }



        bus.game_pak.scenes[state.current_scene].load(
            &mut bus.ppu,
            &mut bus.apu,
            &mut bus.game_pak.assets,
        );
        
        //println!("{:?}", bus.game_pak.scenes[state.current_scene].data_entities);

        match &state.current_scene {
            0 => {
                run_title_screen(bus, &mut events, &mut texture, &mut vid, &mut audio_queue, &mut state);
                state.current_scene = 1;
            }
            1 => {
                run_attract_mode(bus, &mut events, &mut texture, &mut vid, &mut audio_queue, &mut state);
                state.current_scene = 2;
            }
            2 => {
                state.event = String::from("nation_select");
                run_nation_select(
                    bus,
                    &mut events,
                    &mut texture,
                    &mut vid,
                    &mut audio_queue,
                    &mut state,
                );
                state.current_scene = 3;
            }
            3 => {
                state.event = String::from("event_select");
                run_event_select(
                    bus,
                    &mut events,
                    &mut texture,
                    &mut vid,
                    &mut audio_queue,
                    &mut state,
                );

                state.current_scene = state.events.get(&state.event).unwrap().scene;
            }
            4 => {
                
                run_medal_cere(
                    bus,
                    &mut events,
                    &mut texture,
                    &mut vid,
                    &mut audio_queue,
                    &mut state,
                );
                state.current_scene = 3;
            }
            8 => {
                state.event = String::from("victory");
                run_victory(
                    bus,
                    &mut events,
                    &mut texture,
                    &mut vid,
                    &mut audio_queue,
                    &mut state,
                );

                state.current_scene = 2;
                repeat = true;
            }
            5.. => {
                run_biathlon(
                    bus,
                    &mut events,
                    &mut texture,
                    &mut vid,
                    &mut audio_queue,
                    &mut state,
                );
                state.current_scene = 4;
            }
            _ => {
                break 'main;
            }
        }
    }

    Ok(())
}
