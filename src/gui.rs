use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;
use sdl2::pixels::Color;

use specs::prelude::*;

use super::{State, map::Map, Position, Renderable};


const TILEMAP_TILE: u32 = 16;
const TILE_SIZE: u32 = 32;
const MAP_SIZE: u32 = 15;

const BG_COLOR: Color = Color::RGB(11, 32, 39);
const DARK_BG_COLOR: Color = Color::RGB(1, 22, 29);
const LIGHT_BG_COLOR: Color = Color::RGB(64, 121, 140);

#[derive(PartialEq)]
pub enum GuiMode { Unit, Log }

pub const MODES: [(GuiMode, u32, &str); 2] = [
    (GuiMode::Unit, 140, "Unit"),
    (GuiMode::Log, 9, "Log")
];


pub fn render(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    canvas.set_draw_color(BG_COLOR.clone());
    canvas.clear();

    draw_map(canvas, state, tileset);
    draw_unit_list(canvas, state, tileset);
    draw_menu(canvas, state, tileset);
    draw_statusline(canvas, state, tileset);

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


fn draw_map(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    let (width, height) = canvas.output_size().unwrap();

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

    for (render, pos) in (&renderables, &positions).join() {
        if pos.x >= MAP_SIZE && pos.y >= MAP_SIZE {
            continue;
        }

        tileset.set_color_mod(render.color.0, render.color.1, render.color.2);
        draw_tile(canvas, tileset, pos.x, pos.y, render.glyph);
    }
}

fn draw_unit_list(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {}

fn draw_menu(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    let (width, height) = canvas.output_size().unwrap();

    canvas.set_draw_color(DARK_BG_COLOR.clone());
    canvas.fill_rect(Rect::new((TILE_SIZE * MAP_SIZE) as i32, 0, width - MAP_SIZE * TILE_SIZE, TILE_SIZE)).unwrap();

    for (i, (mode, icon_idx, _name)) in MODES.iter().enumerate() {
        if i == state.gui_mode_index {
            canvas.set_draw_color(BG_COLOR.clone());
            canvas.fill_rect(Rect::new((TILE_SIZE * (MAP_SIZE + i as u32 * 3)) as i32, 0, 3 * TILE_SIZE, TILE_SIZE)).unwrap();
            tileset.set_color_mod(200, 200, 200);
        } else {
            tileset.set_color_mod(125, 125, 125);
        }
        draw_tile(canvas, tileset, MAP_SIZE + i as u32 * 3 + 1, 0, *icon_idx);
    }

    match MODES[state.gui_mode_index].0 {
        GuiMode::Log => {
            draw_log(canvas, state, tileset, MAP_SIZE, 1);
        }
        _ => {}
    }
}

fn draw_log(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture, x: u32, y: u32) {
    tileset.set_color_mod(100, 200, 200);
    for (i, entry) in state.log.entries.iter().rev().enumerate() {
        draw_text(canvas, tileset, x, y + i as u32, entry);
    }
}

fn draw_statusline(canvas: &mut WindowCanvas, state: &mut State, tileset: &mut Texture) {
    let (width, height) = canvas.output_size().unwrap();
    canvas.set_draw_color(DARK_BG_COLOR.clone());
    canvas.fill_rect(Rect::new(0, (height - TILE_SIZE) as i32, width, TILE_SIZE)).unwrap();

    tileset.set_color_mod(200, 200, 200);
    draw_tile_real_xy(canvas, tileset, 0, height - TILE_SIZE, 7);
    draw_text_real_xy(canvas, tileset, 2 * TILE_SIZE, height - TILE_SIZE, MODES[state.gui_mode_index].2);
}
