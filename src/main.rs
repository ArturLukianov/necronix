mod gui;
mod gamelog;
mod map;
mod components;
mod mission_system;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{InitFlag, LoadTexture};
use specs::prelude::*;
use std::time::Duration;
use rand::Rng;

use components::*;
use mission_system::MissionSystem;

pub const TICK_SIZE: u32 = 13;


pub struct State {
    log: gamelog::Gamelog,
    ecs: World,
    selected_unit_index: usize
}


fn main() {
    let ctx = sdl2::init().unwrap();

    let video_subsystem = ctx.video().unwrap();
    let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG).unwrap();

    let window = video_subsystem.window("Necronix", 0, 0)
                                .position_centered()
                                .fullscreen_desktop()
                                .build()
                                .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();
    let mut tileset = texture_creator.load_texture("./resources/16x16-RogueYun-AgmEdit.png").unwrap();

    let mut gui = gui::GUI { canvas: canvas, tileset: tileset, menu: gui::GuiMenu::MainMenu(gui::MainMenuButton::Start) };

    let mut state = State{
        ecs: World::new(),
        log: gamelog::Gamelog{ entries: vec!["Welcome to necronix!".to_string()] },
        selected_unit_index: 0
    };

    let map = map::Map::new(15, 15);

    state.ecs.insert(map);

    state.ecs.register::<Renderable>();
    state.ecs.register::<Position>();
    state.ecs.register::<Unit>();
    state.ecs.register::<Name>();
    state.ecs.register::<BlocksTile>();

    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        state.ecs.create_entity()
                 .with(Position{ x: rng.gen_range(0..15), y: rng.gen_range(0..15) })
                 .with(Renderable{ glyph: 139 + rng.gen_range(0..3), color: (50 * rng.gen_range(0..3), 50 * rng.gen_range(0..3), 50 * rng.gen_range(0..3)) })
                 .with(Unit{ mission: Mission::GoTo(rng.gen_range(0..15), rng.gen_range(0..15)) })
                 .build();
    }

    let mut events = ctx.event_pump().unwrap();
    let mut mission_system = MissionSystem {};

    ctx.mouse().show_cursor(false);

    let mut tick = 0;

    'running: loop {
        let units = state.ecs.read_storage::<Unit>().count();

        match gui.menu {
            gui::GuiMenu::GameMenu(tab) => {
                for event in events.poll_iter() {
                    match event {
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } => {
                            break 'running
                        },
                        Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
                            gui.menu = gui::GuiMenu::GameMenu(tab.next());
                        },
                        Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                            state.selected_unit_index = (state.selected_unit_index + 1) % units;
                        },
                        Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                            if state.selected_unit_index == 0 { state.selected_unit_index = units - 1; }
                            else { state.selected_unit_index = state.selected_unit_index - 1; }
                        },
                        Event::KeyDown { keycode: Some(Keycode::M), repeat: false, .. } => {
                            let mut units = state.ecs.write_storage::<Unit>();
                            for (i, unit) in (&mut units).join().enumerate() {
                                if i == state.selected_unit_index {
                                    unit.mission = Mission::GoTo(rng.gen_range(0..15), rng.gen_range(0..15));
                                }
                            }
                        }
                        _ => {}
                    }
                }

                tick = (tick + 1) % TICK_SIZE;
                if tick == 0 {
                    mission_system.run_now(&mut state.ecs);
                    state.ecs.maintain();
                }
            },
            gui::GuiMenu::MainMenu(button) => {
                for event in events.poll_iter() {
                    match event {
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } => {
                            break 'running
                        },
                        Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                            gui.menu = gui::GuiMenu::MainMenu(button.next());
                        },
                        Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                            gui.menu = gui::GuiMenu::MainMenu(button.prev());
                        },
                        Event::KeyDown { keycode: Some(Keycode::Return), repeat: false, .. } => {
                            gui.menu = button.get_menu();
                        }
                        _ => {}
                    }
                }
            },
            _ => {
                for event in events.poll_iter() {
                    match event {
                        Event::Quit {..} => {
                            break 'running
                        },
                        Event::KeyDown { keycode: Some(Keycode::Escape), repeat: false, .. } => {
                            gui.menu = gui::GuiMenu::MainMenu(gui::MainMenuButton::Start)
                        },
                        _ => {}
                    }
                }
            }
        }

        gui.render(&mut state);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    ctx.mouse().show_cursor(true);
}
