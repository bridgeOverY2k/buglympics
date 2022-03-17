use std::collections::HashSet;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;


use lentsys::control::PadControl;
use buglympics::{BlSpy, Native};

pub mod native;
use native::NativeVideo;

extern crate sdl2;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::audio::AudioSpecDesired;
use sdl2::pixels::PixelFormatEnum;
use sdl2::render::Texture;
use sdl2::render::TextureAccess;

fn main() -> Result<(), String>{
    // SDL init
    let sdl_context = sdl2::init()?;
    let mut events = sdl_context.event_pump().unwrap();

    // video
    let mut vid = NativeVideo::new(
        &sdl_context,
        String::from("Winter Buglympics / SPYDER"),
        320,
        240,
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

    // audio
    let sample_rate = 44100.0;
    let audio_subsystem = sdl_context.audio().unwrap();
    let desired_spec = AudioSpecDesired {
        freq: Some(sample_rate as i32),
        channels: Some(1), // mono
        samples: None,     // default sample size
    };
    let audio_queue = audio_subsystem.open_queue::<f32, _>(None, &desired_spec)?;

    // start game
    let mut buffer: Vec<u8> = vec![];
    let input_file = String::from("./web/buglympics.bin");
    let mut file = File::open(input_file)
        .expect("Failed to open game_pak file");

    file.read_to_end(&mut buffer).expect("Failed to fill buffer");
    let mut game = BlSpy::new(&buffer);
    let mut controller = PadControl::new();

    let timer = Instant::now();
    let mut last = 0.0;
    // main loop
    loop {
        
        // exit?
        for event in events.poll_iter() {
            if let Event::Quit { .. } = event {
                println!("Exiting");
                std::process::exit(0);
            };
        }

        // handle inputs
        let keys: HashSet<Keycode> = events
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();
        
        keys_to_pad(&keys, &mut controller);
        game.set_inputs(&controller);

        // update game
        game.update();

        // render frame
        game.render_image();

        // render audio
        let elapsed = timer.elapsed().as_secs_f32();
        let time_delta = elapsed - last;
        game.render_audio(time_delta);

        last = elapsed;

        // grab and set av outputs
        let image_data = game.get_image();
        let audio_data = game.get_audio();
        vid.render_frame(image_data, &mut texture);
        audio_queue.queue(audio_data);
        audio_queue.resume();

        controller.reset();
        
    }
    Ok(())
}

fn keys_to_pad(keys: &HashSet<Keycode>, controller: &mut PadControl){

    if keys.contains(&Keycode::Left){
        controller.left = 255;
    }

    if keys.contains(&Keycode::Right){
        controller.right = 255;
    }

    if keys.contains(&Keycode::Up){
        controller.up = 255;
    }

    if keys.contains(&Keycode::Down){
        controller.down = 255;
    }

    if keys.contains(&Keycode::Return){
        controller.start = 255;
    }

    if keys.contains(&Keycode::Q){
        controller.b = 255;
    }

    if keys.contains(&Keycode::Z){
        controller.a = 255;
    }
    
    if keys.contains(&Keycode::A){
        controller.x = 255;
    }
}
