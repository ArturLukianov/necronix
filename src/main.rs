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
    gui_mode_index: usize,
    log: gamelog::Gamelog,
    ecs: World
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

    let mut state = State{
        gui_mode_index: 0,
        ecs: World::new(),
        log: gamelog::Gamelog{ entries: vec!["Welcome to necronix!".to_string()] },
    };

    let map = map::Map::new(15, 15);

    state.ecs.insert(map);

    state.ecs.register::<Renderable>();
    state.ecs.register::<Position>();
    state.ecs.register::<Unit>();

    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        state.ecs.create_entity()
                 .with(Position{ x: rng.gen_range(0..15), y: rng.gen_range(0..15) })
                 .with(Renderable{ glyph: '%' as u32, color: (0, 100, 100) })
                 .with(Unit{})
                 .build();
    }

    let mut events = ctx.event_pump().unwrap();

    ctx.mouse().show_cursor(false);

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown { keycode: Some(Keycode::Tab), repeat: false, .. } => {
                    state.gui_mode_index = (state.gui_mode_index + 1) % gui::MODES.len();
                }
                _ => {}
            }
        }
        gui::render(&mut canvas, &mut state, &mut tileset);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    ctx.mouse().show_cursor(true);
}
