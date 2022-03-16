extern crate sdl2;
use sdl2::render::Texture;

pub struct NativeVideo {
  pub width: u32,
  pub height: u32,
  pub canvas: sdl2::render::WindowCanvas
}

impl NativeVideo {
  pub fn new(
    sdl_context: &sdl2::Sdl, 
    window_name: String, 
    width: u32, 
    height: u32 
  ) -> NativeVideo {


    let video_subsystem = sdl_context.video().unwrap();
    
    let window = video_subsystem
        .window(
            &window_name,
            width * 2,
            height * 2,
        )
        //.fullscreen()
        .position_centered()
        .build()
        .map_err(|e| e.to_string()).unwrap();

    let canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    NativeVideo {
      width,
      height,
      canvas
    }    
  }

  pub fn render_frame(
    &mut self,
    ppu_vals: &Vec<u8>,
    texture: &mut Texture
  ) {
    texture.update(None, ppu_vals, self.width as usize * 4).unwrap();
    self.canvas.clear();
    self.canvas.copy(&texture, None, None).unwrap();
    self.canvas.present();
  }
}

pub fn native_render_frame(
  ppu_vals: Vec<u8>,
  ppu_res_width: usize,
  texture: &mut Texture,
  canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
  texture.update(None, &ppu_vals, ppu_res_width).unwrap();
  canvas.clear();
  canvas.copy(&texture, None, None).unwrap();
  canvas.present();
}