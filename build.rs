use lentsys::game_pak::asset::AssetCollection;
use lentsys::game_pak::scene::Scene;
use lentsys::game_pak::scene::SceneState;
use lentsys::game_pak::GamePak;
use lentsys::io::Prepare;

pub fn main(){
  pack_game();
}

pub fn pack_game() {
  // Prepare game asset binary from raw assets
  let mut gp = GamePak {
    name: String::from("Buglympics and Spyder"),
    assets: AssetCollection::new(),
    scenes: vec![
      Scene {
        name: String::from("title_screen"),
        tile_set_names: vec![String::from("title_screen"), String::from("title_screen_hh")],
        tile_map_names: vec![String::from("title_screen")],
        data_entities: vec![],
        sample_names: vec![String::from("frozen_lake")],
        music_names: vec![],
        state: SceneState::INITIAL,
      },
      Scene {
        name: String::from("attract_mode"),
        tile_set_names: vec![],
        tile_map_names: vec![],
        data_entities: vec![],
        sample_names: vec![String::from("credits")],
        music_names: vec![],
        state: SceneState::INITIAL,
      },
      Scene {
        name: String::from("nation_select"),
        tile_set_names: vec![String::from("nation_select"), String::from("tool_select")],
        tile_map_names: vec![String::from("nation_select")],
        data_entities: vec![],
        sample_names: vec![String::from("river")],
        music_names: vec![],
        state: SceneState::INITIAL,
      },
      Scene {
        name: String::from("event_select"),
        tile_set_names: vec![String::from("nation_select"), String::from("tool_select")],
        tile_map_names: vec![String::from("event_select")],
        data_entities: vec![],
        sample_names: vec![],
        music_names: vec![],
        state: SceneState::INITIAL,
      },
      Scene {
        name: String::from("arena"),
        tile_set_names: vec![String::from("arena"), String::from("arena_hh")],
        tile_map_names: vec![String::from("arena")],
        data_entities: vec![],
        sample_names: vec![],
        music_names: vec![],
        state: SceneState::INITIAL,
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
        data_entities: vec![],
        sample_names: vec![String::from("pachyderm")],
        music_names: vec![],
        state: SceneState::INITIAL,
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
        data_entities: vec![],
        sample_names: vec![String::from("pachyderm")],
        music_names: vec![],
        state: SceneState::INITIAL,
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
        data_entities: vec![],
        sample_names: vec![String::from("pachyderm")],
        music_names: vec![],
        state: SceneState::INITIAL,
      },
      Scene {
        name: String::from("victory"),
        tile_set_names: vec![],
        tile_map_names: vec![],
        data_entities: vec![],
        sample_names: vec![String::from("pachyderm")],
        music_names: vec![],
        state: SceneState::INITIAL,
      },
    ],
  };

  // load assets from files
  let _result = gp.assets.gather_assets("./assets/");

  gp.assets.prepare();
  gp.to_binary(&String::from("./buglympics.bin"));
}
