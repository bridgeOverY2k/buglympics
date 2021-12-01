use zingr::game_pak::asset::Asset;
use zingr::game_pak::asset::AssetCollection;
use zingr::game_pak::asset::AssetType;
use zingr::game_pak::scene::Scene;
use zingr::game_pak::scene::SceneState;
use zingr::game_pak::GamePak;

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
  gp.assets.manifest.append(&mut vec![
    Asset {
      name: String::from("title_screen"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/title_screen.tmx"),
    },
    Asset {
      name: String::from("frozen_lake"),
      asset_type: AssetType::SampleAsset,
      path: String::from("./assets/1985/frozen_lake_clip.wav"),
    },
    Asset {
      name: String::from("pachyderm"),
      asset_type: AssetType::SampleAsset,
      path: String::from("./assets/1985/pachyderm_clip.wav"),
    },
    Asset {
      name: String::from("river"),
      asset_type: AssetType::SampleAsset,
      path: String::from("./assets/1985/river_clip.wav"),
    },
    Asset {
      name: String::from("credits"),
      asset_type: AssetType::SampleAsset,
      path: String::from("./assets/1985/credits_clip.wav"),
    },
    Asset {
      name: String::from("crosscountry"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/crosscountry_biathlon_01.tmx"),
    },
    Asset {
      name: String::from("craggy"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/craggy_biathlon_02.tmx"),
    },
    Asset {
      name: String::from("downhill"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/downhill_biathlon_01.tmx"),
    },
    Asset {
      name: String::from("attract_bg_01"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/attract_bg01.tmx"),
    },
    Asset {
      name: String::from("cut_01_arrival"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/cut_scene_01_arrival.tmx"),
    },
    Asset {
      name: String::from("cut_02_park"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/cut_02_park.tmx"),
    },
    Asset {
      name: String::from("nation_select"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/nation_select.tmx"),
    },
    Asset {
      name: String::from("event_select"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/event_select.tmx"),
    },
    Asset {
      name: String::from("arena"),
      asset_type: AssetType::TileMapAsset,
      path: String::from("./assets/1985/arena_map_01.tmx"),
    },
    Asset {
      name: String::from("winter_set"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1985/winter_set.png"),
    },
    Asset {
      name: String::from("winter_set_hh"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1986/winter_set_hh.png"),
    },
    Asset {
      name: String::from("title_screen"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1985/title_screen.png"),
    },
    Asset {
      name: String::from("title_screen_hh"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1986/title_screen.png"),
    },
    Asset {
      name: String::from("buglympics_shot_01"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1985/buglympics_shot_01_concept.png"),
    },
    Asset {
      name: String::from("buglympics_shot_02"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1985/cut_01_arrival.png"),
    },
    Asset {
      name: String::from("buglympics_victory"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1985/buglympics_victory.png"),
    },
    Asset {
      name: String::from("spyder_shot_01"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1986/spyder_shot_01_briefing.png"),
    },
    Asset {
      name: String::from("spyder_shot_02"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1986/cut_01_arrival.png"),
    },
    Asset {
      name: String::from("spyder_shot_03"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1986/spyder_shot_03_sneak.png"),
    },
    Asset {
      name: String::from("spyder_victory"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1986/spyder_victory.png"),
    },
    Asset {
      name: String::from("cut_02_park"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1985/cut_02_park.png"),
    },
    Asset {
      name: String::from("nation_select"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1985/nation_select.png"),
    },
    Asset {
      name: String::from("tool_select"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1986/tool_select.png"),
    },
    Asset {
      name: String::from("arena"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1985/arena.png"),
    },
    Asset {
      name: String::from("arena_hh"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("./assets/1986/arena_hh.png"),
    },
    Asset {
      name: String::from("ant_walk"),
      asset_type: AssetType::TileSetAsset(32, 48),
      path: String::from("./assets/1986/ant_anim_hh.png"),
    },
    Asset {
      name: String::from("ant_ski"),
      asset_type: AssetType::TileSetAsset(32, 48),
      path: String::from("./assets/1985/ant_ski.png"),
    },
    Asset {
      name: String::from("noto_sans"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("assets/1985/Noto_Sans_Mono.bmp"),
    },
    Asset {
      name: String::from("start_font"),
      asset_type: AssetType::TileSetAsset(16, 16),
      path: String::from("assets/1985/press_start_font16.bmp"),
    },
    Asset {
      name: String::from("start_font_small"),
      asset_type: AssetType::TileSetAsset(8, 8),
      path: String::from("assets/1985/press_start_font8.bmp"),
    },
  ]);

  // load assets from files
  gp.assets.prepare();
  gp.to_binary(&String::from("./buglympics.bin"));
}
