mod gui;
mod gamelog;
mod map;
mod components;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{InitFlag, LoadTexture};
use specs::prelude::*;
use std::time::Duration;
use rand::Rng;

use components::*;


pub struct State {
    gui_game_mode_index: usize,
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
        gui_game_mode_index: 0,
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
                 .with(Unit{})
                 .build();
    }

    let mut events = ctx.event_pump().unwrap();

    ctx.mouse().show_cursor(false);

    'running: loop {
        let units = state.ecs.read_storage::<Unit>().count();

        match gui.menu {
            gui::GuiMenu::GameMenu(_) => {
                for event in events.poll_iter() {
                    match event {
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running
                        },
                        Event::KeyDown { keycode: Some(Keycode::Tab), .. } => {
                            state.gui_game_mode_index = (state.gui_game_mode_index + 1) % gui::GameMenuTabVariants;
                        },
                        Event::KeyDown { keycode: Some(Keycode::D), .. } => {
                            state.selected_unit_index = (state.selected_unit_index + 1) % units;
                        },
                        Event::KeyDown { keycode: Some(Keycode::A), .. } => {
                            if state.selected_unit_index == 0 { state.selected_unit_index = units - 1; }
                            else { state.selected_unit_index = state.selected_unit_index - 1; }
                        }
                        _ => {}
                    }
                }
            },
            gui::GuiMenu::MainMenu(_) => {
                for event in events.poll_iter() {
                    match event {
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'running
                        },
                        Event::KeyDown { keycode: Some(Keycode::D), repeat: false, .. } => {
                            state.selected_unit_index = (state.selected_unit_index + 1) % units;
                        },
                        Event::KeyDown { keycode: Some(Keycode::A), repeat: false, .. } => {
                            if state.selected_unit_index == 0 { state.selected_unit_index = units - 1; }
                            else { state.selected_unit_index = state.selected_unit_index - 1; }
                        },
                        Event::KeyDown { keycode: Some(Keycode::Return), repeat: false, .. } => {
                            gui.menu = gui::GuiMenu::GameMenu(gui::GameMenuTab::Unit);
                        }
                        _ => {}
                    }
                }
            },
            _ => {}
        }

        gui.render(&mut state);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    ctx.mouse().show_cursor(true);
}
