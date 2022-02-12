use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::Rect;
use sdl2::pixels::Color;


const TILEMAP_TILE: u32 = 16;
const TILE_SIZE: u32 = 32;
const MAP_SIZE: u32 = 15;

const BG_COLOR: Color = Color::RGB(11, 32, 39);
const DARK_BG_COLOR: Color = Color::RGB(1, 22, 29);
const LIGHT_BG_COLOR: Color = Color::RGB(64, 121, 140);

enum GuiMode { Unit, Log }


pub fn render(canvas: &mut WindowCanvas, tileset: &mut Texture) {
    canvas.set_draw_color(BG_COLOR.clone());
    canvas.clear();

    draw_map(canvas, tileset);
    draw_unit_list(canvas);
    draw_menu(canvas, tileset);

    canvas.present();
}

fn tile_rect(idx: u32) -> Rect {
    let x = idx % 16;
    let y = idx / 16;
    Rect::new((TILEMAP_TILE * x) as i32, (TILEMAP_TILE * y) as i32, TILEMAP_TILE, TILEMAP_TILE)
}


fn draw_map(canvas: &mut WindowCanvas, tileset: &mut Texture) {
    let (width, height) = canvas.output_size().unwrap();

    canvas.set_draw_color(DARK_BG_COLOR.clone());
    canvas.fill_rect(Rect::new(0, 0, TILE_SIZE * MAP_SIZE, TILE_SIZE * MAP_SIZE)).unwrap();
    for x in 0..15 {
        for y in 0..15 {
            tileset.set_color_mod(200, 200, 200);
            canvas.copy(tileset, Rect::new(0 as i32, 0, TILEMAP_TILE, TILEMAP_TILE),
                        Rect::new((x * TILE_SIZE) as i32, (y * TILE_SIZE) as i32, TILE_SIZE, TILE_SIZE)).unwrap();
        }
    }
}

fn draw_unit_list(canvas: &mut WindowCanvas) {
    let (width, height) = canvas.output_size().unwrap();

}

fn draw_menu(canvas: &mut WindowCanvas, tileset: &mut Texture) {
    let (width, height) = canvas.output_size().unwrap();

    canvas.set_draw_color(DARK_BG_COLOR.clone());

    for i in 0..3 {
        canvas.fill_rect(Rect::new((TILE_SIZE * (MAP_SIZE + i)) as i32, 0, 1 * TILE_SIZE, 1 * TILE_SIZE)).unwrap();

        tileset.set_color_mod(125, 125, 125);
        canvas.copy(tileset, tile_rect(2 + i),
                    Rect::new(((MAP_SIZE + i) * TILE_SIZE) as i32, 0, TILE_SIZE, TILE_SIZE)).unwrap();
    }
}
