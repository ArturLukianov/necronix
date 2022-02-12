mod gui;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::image::{InitFlag, LoadTexture};
use std::time::Duration;

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



    let mut events = ctx.event_pump().unwrap();

    'running: loop {
        for event in events.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        gui::render(&mut canvas, &mut tileset);
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
