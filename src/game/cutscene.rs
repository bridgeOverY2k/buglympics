use lentsys::lentsys::LentSysBus;
use lentsys::ui::text::TextBox;

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
