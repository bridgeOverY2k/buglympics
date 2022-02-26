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

pub mod game;
use game::attract_mode::run_attract_mode;
use game::biathlon::run_biathlon;
use game::event_select::run_event_select;
use game::title_screen::run_title_screen;
use game::medal_cere::run_medal_cere;
use game::nation_select::run_nation_select;
use game::victory::run_victory;
use game::native::NativeVideo;
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

    let world = game::WorldState {
        gravity: 0.5,
        collision_set: lentsys::ppu::attr::AttrSet {
            tile_set_id: 0,
            tiles: std::collections::HashMap::new(),
        },
    };

    let buglympics = BuglympicsState {
        nation: String::from(""),
        events: vec![
            (
                String::from("CROSS-COUNTRY BIATHLON"),
                BuglympicsEvent {
                    start_line: [100, 100],
                    finish_line: [189 * 16, 26 * 16],
                },
            ),
            (
                String::from("DOWNHILL BIATHLON"),
                BuglympicsEvent {
                    start_line: [100, 100],
                    finish_line: [90 * 16, 200 * 16],
                },
            ),
            (
                String::from("CRAGGY BIATHLON"),
                BuglympicsEvent {
                    start_line: [100, 100],
                    finish_line: [141 * 16, 24 * 16],
                },
            ),
        ]
        .into_iter()
        .collect(),
        medals: vec![
            (
                String::from("CROSS-COUNTRY BIATHLON"),
                MedalStanding {
                    event: String::from("CROSS-COUNTRY BIATHLON"),
                    medals: vec![
                        BuglympicsEventRecord {
                            nation: String::from("Beehama"),
                            event: String::from("biathlon"),
                            time: 6.0,
                        },
                        BuglympicsEventRecord {
                            nation: String::from("Beehama"),
                            event: String::from("biathlon"),
                            time: 10.0,
                        },
                        BuglympicsEventRecord {
                            nation: String::from("Beehama"),
                            event: String::from("biathlon"),
                            time: 16.0,
                        },
                    ],
                },
            ),
            (
                String::from("DOWNHILL BIATHLON"),
                MedalStanding {
                    event: String::from("DOWNHILL BIATHLON"),
                    medals: vec![
                        BuglympicsEventRecord {
                            nation: String::from("Beehama"),
                            event: String::from("biathlon"),
                            time: 10.0,
                        },
                        BuglympicsEventRecord {
                            nation: String::from("Beehama"),
                            event: String::from("biathlon"),
                            time: 14.0,
                        },
                        BuglympicsEventRecord {
                            nation: String::from("Beehama"),
                            event: String::from("biathlon"),
                            time: 24.0,
                        },
                    ],
                },
            ),
            (
                String::from("CRAGGY BIATHLON"),
                MedalStanding {
                    event: String::from("CRAGGY BIATHLON"),
                    medals: vec![
                        BuglympicsEventRecord {
                            nation: String::from("Beehama"),
                            event: String::from("biathlon"),
                            time: 12.0,
                        },
                        BuglympicsEventRecord {
                            nation: String::from("Beehama"),
                            event: String::from("biathlon"),
                            time: 16.0,
                        },
                        BuglympicsEventRecord {
                            nation: String::from("Beehama"),
                            event: String::from("biathlon"),
                            time: 25.0,
                        },
                    ],
                },
            ),
        ]
        .into_iter()
        .collect(),
    };

    let spyder = SpyderState {
        events: vec![
            (
                String::from("CROSS-COUNTRY BIATHLON"),
                SpyderEvent {
                    start_line: [100, 100],
                    targets: vec![],
                    time_limit: 30.0
                },
            ),
            (
                String::from("DOWNHILL BIATHLON"),
                SpyderEvent {
                    start_line: [100, 100],
                    targets: vec![],
                    time_limit: 30.0
                },
            ),
            (
                String::from("CRAGGY BIATHLON"),
                SpyderEvent {
                    start_line: [100, 100],
                    targets: vec![],
                    time_limit: 30.0
                },
            ),
        ].into_iter().collect(),
        results: vec![].into_iter().collect(),
    };

    let mut state = GameState {
        game: GameMode::Buglympics,
        swap_cooldown: 0,
        current_scene: 0,
        event: String::from("title_screen"),
        last_event_success: false,
        events: vec![
            (
                String::from("title_screen"),
                SceneMap {
                    scene: 0,
                    bl_tm_pal: vec![(0, 0), (1, 2)].into_iter().collect(),
                    bl_sp_pal: vec![].into_iter().collect(),
                    bl_sp_ts: vec![].into_iter().collect(),
                    spy_tm_ts: vec![(0, 1)].into_iter().collect(),
                    spy_tm_pal: vec![(0, 1), (1, 1)].into_iter().collect(),
                    spy_sp_pal: vec![].into_iter().collect(),
                    spy_sp_ts: vec![].into_iter().collect(),
                    ..SceneMap::default()
                },
            ),
            (
                String::from("nation_select"),
                SceneMap {
                    scene: 2,
                    both_complete: false,
                    bl_tm_ts: vec![(0, 0)].into_iter().collect(),
                    bl_tm_pal: vec![(0, 0), (1, 2), (2, 2), (3, 2), (4, 2)].into_iter().collect(),
                    bl_sp_ts: vec![(0, 2)].into_iter().collect(),
                    bl_sp_pal: vec![(0, 2)].into_iter().collect(),

                    spy_tm_ts: vec![(0, 1)].into_iter().collect(),
                    spy_tm_pal: vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)].into_iter().collect(),
                    spy_sp_ts: vec![(0, 2)].into_iter().collect(),
                    spy_sp_pal: vec![(0, 2)].into_iter().collect(),
                },
            ),
            (
                String::from("event_select"),
                SceneMap {
                    scene: 3,
                    both_complete: false,
                    bl_tm_ts: vec![(0, 0)].into_iter().collect(),
                    bl_tm_pal: vec![(0, 0), (1, 2), (2, 2), (3, 2), (4, 2)].into_iter().collect(),
                    bl_sp_ts: vec![(0, 2)].into_iter().collect(),
                    bl_sp_pal: vec![(0, 2)].into_iter().collect(),

                    spy_tm_ts: vec![(0, 1)].into_iter().collect(),
                    spy_tm_pal: vec![(0, 1), (1, 1), (2, 1), (3, 1), (4, 1)].into_iter().collect(),
                    spy_sp_ts: vec![(0, 2)].into_iter().collect(),
                    spy_sp_pal: vec![(0, 2)].into_iter().collect(),
                },
            ),
            (
                String::from("medal_cere"),
                SceneMap {
                    scene: 4,
                    both_complete: false,
                    bl_tm_ts: vec![(0, 0)].into_iter().collect(),
                    bl_tm_pal: vec![(0, 0)].into_iter().collect(),
                    bl_sp_ts: vec![].into_iter().collect(),
                    bl_sp_pal: vec![].into_iter().collect(),

                    spy_tm_ts: vec![(0, 1)].into_iter().collect(),
                    spy_tm_pal: vec![(0, 1)].into_iter().collect(),
                    spy_sp_ts: vec![].into_iter().collect(),
                    spy_sp_pal: vec![].into_iter().collect(),
                },
            ),
            (
                String::from("CROSS-COUNTRY BIATHLON"),
                SceneMap {
                    scene: 5,
                    ..SceneMap::default()
                },
            ),
            (
                String::from("DOWNHILL BIATHLON"),
                SceneMap {
                    scene: 6,
                    ..SceneMap::default()
                },
            ),(
                String::from("CRAGGY BIATHLON"),
                SceneMap {
                    scene: 7,
                    ..SceneMap::default()
                },
            ),
            (
                String::from("victory"),
                SceneMap {
                    scene: 8,
                    both_complete: false,
                    bl_tm_ts: vec![].into_iter().collect(),
                    bl_tm_pal: vec![].into_iter().collect(),
                    bl_sp_ts: vec![].into_iter().collect(),
                    bl_sp_pal: vec![].into_iter().collect(),

                    spy_tm_ts: vec![].into_iter().collect(),
                    spy_tm_pal: vec![].into_iter().collect(),
                    spy_sp_ts: vec![].into_iter().collect(),
                    spy_sp_pal: vec![].into_iter().collect(),
                },
            ),
        ]
        .into_iter()
        .collect(),
        world,
        buglympics,
        spyder,
    };

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
