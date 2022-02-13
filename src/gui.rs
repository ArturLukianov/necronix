use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use specs::prelude::*;

use super::{State, map::Map, Position, Renderable, Unit, Name};


const TILEMAP_TILE: u32 = 16;
const TILE_SIZE: u32 = 32;
const MAP_SIZE: u32 = 15;

const BG_COLOR: Color = Color::RGB(11, 32, 39);
const DARK_BG_COLOR: Color = Color::RGB(1, 22, 29);
const LIGHT_BG_COLOR: Color = Color::RGB(64, 121, 140);

#[derive(PartialEq)]
pub enum GuiMenu { MainMenu, HelpMenu, GameMenu }

#[derive(PartialEq)]
pub enum GuiMode { Unit, Log }

pub const MODES: [(GuiMode, u32, &str); 2] = [
    (GuiMode::Unit, 140, "Unit"),
    (GuiMode::Log, 9, "Log")
];


pub fn render(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    canvas.set_draw_color(BG_COLOR.clone());
    canvas.clear();

    match state.gui_menu {
        GuiMenu::GameMenu => {
            draw_map(canvas, state, tileset);
            draw_unit_list(canvas, state, tileset);
            draw_menu(canvas, state, tileset);
            draw_statusline(canvas, state, tileset);
        },
        GuiMenu::MainMenu => {
            draw_main_menu(canvas, state, tileset);
        },
        _ => {}
    }

    canvas.present();
}


fn tile_rect(idx: u32) -> Rect {
    let x = idx % 16;
    let y = idx / 16;
    Rect::new((TILEMAP_TILE * x) as i32, (TILEMAP_TILE * y) as i32, TILEMAP_TILE, TILEMAP_TILE)
}

fn draw_tile(canvas: &mut WindowCanvas, tileset: &mut Texture, x: u32, y: u32, tile_idx: u32) {
    canvas.copy(tileset, tile_rect(tile_idx),
                Rect::new((x * TILE_SIZE) as i32, (y * TILE_SIZE) as i32, TILE_SIZE, TILE_SIZE)).unwrap();
}

fn draw_tile_real_xy(canvas: &mut WindowCanvas, tileset: &mut Texture, x: u32, y: u32, tile_idx: u32) {
    canvas.copy(tileset, tile_rect(tile_idx),
                Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE)).unwrap();
}

fn draw_text<T: ToString>(canvas: &mut WindowCanvas, tileset: &mut Texture, x: u32, y: u32, text: T) {
    let string: String = text.to_string();
    for (i, char) in string.chars().enumerate() {
        draw_tile(canvas, tileset, x + i as u32, y, char as u32);
    }
}

fn draw_text_real_xy<T: ToString>(canvas: &mut WindowCanvas, tileset: &mut Texture, x: u32, y: u32, text: T) {
    let string: String = text.to_string();
    for (i, char) in string.chars().enumerate() {
        draw_tile_real_xy(canvas, tileset, x + TILE_SIZE * i as u32, y, char as u32);
    }
}


fn draw_main_menu(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    let (width, height) = canvas.output_size().unwrap();

    let title = "Necronix";
    let button_text = "Start";
    tileset.set_color_mod(255, 255, 255);
    draw_text_real_xy(canvas, tileset, width / 2 - title.len() as u32 * TILE_SIZE / 2, height / 2 - TILE_SIZE / 2 - TILE_SIZE * 2, title);
    tileset.set_color_mod(100, 100, 100);
    draw_tile_real_xy(canvas, tileset, width / 2 - TILE_SIZE / 2 - TILE_SIZE * 2, height / 2 + TILE_SIZE / 2 - TILE_SIZE, '<' as u32);
    draw_tile_real_xy(canvas, tileset, width / 2 - TILE_SIZE / 2 + TILE_SIZE * 2, height / 2 + TILE_SIZE / 2 - TILE_SIZE, '>' as u32);
    tileset.set_color_mod(100, 0, 200);
    draw_tile_real_xy(canvas, tileset, width / 2 - TILE_SIZE / 2, height / 2 + TILE_SIZE / 2 - TILE_SIZE, 140);
    tileset.set_color_mod(100, 0, 200);
    draw_text_real_xy(canvas, tileset, width / 2 - button_text.len() as u32 * TILE_SIZE / 2, height / 2 - TILE_SIZE / 2 + TILE_SIZE * 2, button_text);
}

fn draw_map(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    let map = state.ecs.fetch::<Map>();

    canvas.set_draw_color(DARK_BG_COLOR.clone());
    canvas.fill_rect(Rect::new(0, 0, TILE_SIZE * MAP_SIZE, TILE_SIZE * MAP_SIZE)).unwrap();
    tileset.set_color_mod(100, 100, 100);
    let mut y = 0;
    let mut x = 0;
    for tile in map.tiles.iter() {
        if *tile == 1 {
            draw_tile(canvas, tileset, x, y, 0xdb);
        }

        x += 1;
        if x == map.width || x == MAP_SIZE {
            x = 0;
            y += 1;
            if y == MAP_SIZE {
                break
            }
        }
    }

    let renderables = state.ecs.read_storage::<Renderable>();
    let positions = state.ecs.read_storage::<Position>();

    for (i, (render, pos)) in (&renderables, &positions).join().enumerate() {
        if pos.x >= MAP_SIZE && pos.y >= MAP_SIZE {
            continue;
        }

        if i == state.selected_unit_index {
            tileset.set_color_mod(render.color.0 / 2 * 3, render.color.1 / 2 * 3, render.color.2 / 2 * 3);
        } else {
            tileset.set_color_mod(render.color.0, render.color.1, render.color.2);
        }
        draw_tile(canvas, tileset, pos.x, pos.y, render.glyph);
    }
}

fn draw_unit_list(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    let units = state.ecs.read_storage::<Unit>();
    let renderables = state.ecs.read_storage::<Renderable>();
    let map = state.ecs.fetch::<Map>();

    let mut x = 0;
    let mut y = 0;

    for (i, (_unit, render)) in (&units, &renderables).join().enumerate() {
        if i == state.selected_unit_index {
            tileset.set_color_mod(render.color.0 / 2 * 3, render.color.1 / 2 * 3, render.color.2 / 2 * 3);
        } else {
            tileset.set_color_mod(render.color.0, render.color.1, render.color.2);
        }
        draw_tile(canvas, tileset, x, MAP_SIZE + y, render.glyph);

        x += 1;
        if x == map.width {
            y += 1;
            x = 0;
        }
    }
}

fn draw_menu(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    let (width, height) = canvas.output_size().unwrap();

    let mut renderable: Option<Renderable> = None;
    {
        let units = state.ecs.read_storage::<Unit>();
        let renderables = state.ecs.read_storage::<Renderable>();

        for (i, (_, render)) in (&units, &renderables).join().enumerate() {
            if i == state.selected_unit_index {
                renderable = Some(*render);
                break;
            }
        }
    }

    canvas.set_draw_color(DARK_BG_COLOR.clone());
    canvas.fill_rect(Rect::new((TILE_SIZE * MAP_SIZE) as i32, 0, width - MAP_SIZE * TILE_SIZE, TILE_SIZE)).unwrap();

    for (i, (mode, icon_idx, _name)) in MODES.iter().enumerate() {
        if i == state.gui_game_mode_index {
            canvas.set_draw_color(BG_COLOR.clone());
            canvas.fill_rect(Rect::new((TILE_SIZE * (MAP_SIZE + i as u32 * 3)) as i32, 0, 3 * TILE_SIZE, TILE_SIZE)).unwrap();
            tileset.set_color_mod(200, 200, 200);
        } else {
            tileset.set_color_mod(125, 125, 125);
        }

        if *mode == GuiMode::Unit {
            if let Some(renderable) = renderable {
                if i == state.gui_game_mode_index { tileset.set_color_mod(renderable.color.0 * 2, renderable.color.1 * 2, renderable.color.2 * 2); }
                else { tileset.set_color_mod(renderable.color.0, renderable.color.1, renderable.color.2); }
                draw_tile(canvas, tileset, MAP_SIZE + i as u32 * 3 + 1, 0, renderable.glyph);
            } else {
                draw_tile(canvas, tileset, MAP_SIZE + i as u32 * 3 + 1, 0, *icon_idx);
            }
            continue;
        }

        draw_tile(canvas, tileset, MAP_SIZE + i as u32 * 3 + 1, 0, *icon_idx);
    }

    match MODES[state.gui_game_mode_index].0 {
        GuiMode::Log => {
            draw_log(canvas, state, tileset, MAP_SIZE, 1);
        },
        GuiMode::Unit => {
            draw_unit_info(canvas, state, tileset, MAP_SIZE, 1);
        },
        _ => {}
    }
}

fn draw_log(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture, x: u32, y: u32) {
    tileset.set_color_mod(100, 200, 200);
    for (i, entry) in state.log.entries.iter().rev().enumerate() {
        draw_text(canvas, tileset, x, y + i as u32, entry);
    }
}

fn draw_unit_info(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture, x: u32, y: u32) {
    let units = state.ecs.read_storage::<Unit>();
    let positions = state.ecs.read_storage::<Position>();
    let renderables = state.ecs.read_storage::<Renderable>();
    let entities = state.ecs.entities();
    let names = state.ecs.read_storage::<Name>();

    for (i, (_, entity, position, render)) in (&units, &entities, &positions, &renderables).join().enumerate() {
        if i == state.selected_unit_index {
            tileset.set_color_mod(render.color.0 * 2, render.color.1 * 2, render.color.2 * 2);
            let mut name = "Unnamed";
            draw_text(canvas, tileset, x, y, "Unnamed");
            tileset.set_color_mod(200, 200, 200);
            draw_text(canvas, tileset, x + name.len() as u32 + 1, y, format!("{}:{}", position.x, position.y));

            break;
        }
    }
}

fn draw_statusline(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    let (width, height) = canvas.output_size().unwrap();
    canvas.set_draw_color(DARK_BG_COLOR.clone());
    canvas.fill_rect(Rect::new(0, (height - TILE_SIZE) as i32, width, TILE_SIZE)).unwrap();



    tileset.set_color_mod(200, 200, 200);
    draw_tile_real_xy(canvas, tileset, 0, height - TILE_SIZE, 7);
    draw_text_real_xy(canvas, tileset, 2 * TILE_SIZE, height - TILE_SIZE, MODES[state.gui_game_mode_index].2);
}
