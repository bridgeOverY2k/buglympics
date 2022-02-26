
pub fn main(){
  // only rebuild binary if native
  pack_game();
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub fn pack_game(){
  println!("Game not re-packed")
}

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn pack_game() {
  println!("Packing game")
  use lentsys::game_pak::asset::AssetCollection;
  use lentsys::game_pak::scene::Scene;
  use lentsys::game_pak::scene::SceneState;
  use lentsys::game_pak::GamePak;
  use lentsys::io::Prepare;
  // Prepare game asset binary from raw assets
  let mut gp = GamePak {
    name: String::from("Buglympics and Spyder"),
    assets: AssetCollection::new(),
    scenes: vec![
      Scene {
        name: String::from("title_screen"),
        tile_set_names: vec![String::from("title_screen"), String::from("title_screen_hh")],
        tile_map_names: vec![String::from("title_screen")],
        sample_names: vec![String::from("frozen_lake")],
        ..Default::default()
      },
      Scene {
        name: String::from("attract_mode"),
        sample_names: vec![String::from("credits")],
        ..Default::default()
      },
      Scene {
        name: String::from("nation_select"),
        tile_set_names: vec![String::from("nation_select"), String::from("tool_select")],
        tile_map_names: vec![String::from("nation_select")],
        sample_names: vec![String::from("river")],
        ..Default::default()
      },
      Scene {
        name: String::from("event_select"),
        tile_set_names: vec![String::from("nation_select"), String::from("tool_select")],
        tile_map_names: vec![String::from("event_select")],
        ..Default::default()
      },
      Scene {
        name: String::from("arena"),
        tile_set_names: vec![String::from("arena"), String::from("arena_hh")],
        tile_map_names: vec![String::from("arena")],
        ..Default::default()
      },
      Scene {
        name: String::from("crosscounty"),
        tile_set_names: vec![
          String::from("winter_set"), 
          String::from("start_font_small"),
          String::from("ant_ski"), 
          String::from("winter_set_hh"),
          String::from("ant_walk"),
        ],
        tile_map_names: vec![String::from("crosscountry")],
        sample_names: vec![String::from("pachyderm")],
        ..Default::default()
      },
      Scene {
        name: String::from("downhill"),
        tile_set_names: vec![
          String::from("winter_set"), 
          String::from("start_font_small"),
          String::from("ant_ski"), 
          String::from("winter_set_hh"),
          String::from("ant_walk"),
        ],
        tile_map_names: vec![String::from("downhill")],
        sample_names: vec![String::from("pachyderm")],
        ..Default::default()
      },
      Scene {
        name: String::from("craggy"),
        tile_set_names: vec![
          String::from("winter_set"), 
          String::from("start_font_small"),
          String::from("ant_ski"), 
          String::from("winter_set_hh"),
          String::from("ant_walk"),
        ],
        tile_map_names: vec![String::from("craggy")],
        sample_names: vec![String::from("pachyderm")],
        ..Default::default()
      },
      Scene {
        name: String::from("victory"),
        sample_names: vec![String::from("pachyderm")],
        ..Default::default()
      },
    ],
  };

  // load assets from files
  let _result = gp.assets.gather_assets("./assets/");

  gp.assets.prepare();
  gp.to_binary(&String::from("./buglympics.bin"));
}
