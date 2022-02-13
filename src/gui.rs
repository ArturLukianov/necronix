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
pub enum GuiMenu { MainMenu(MainMenuButton), HelpMenu, GameMenu(GameMenuTab) }

#[derive(PartialEq)]
pub enum MainMenuButton { Start, Help, Credits }
pub const MainMenuButtonVariants : usize = 3;

#[derive(PartialEq, Clone)]
pub enum GameMenuTab { Unit, Log }
pub const GameMenuTabVariants : usize = 2;


pub struct GUI<'a> {
    pub canvas: WindowCanvas,
    pub tileset: Texture<'a>,
    pub menu: GuiMenu
}

fn tile_rect(idx: u32) -> Rect {
    let x = idx % 16;
    let y = idx / 16;
    Rect::new((TILEMAP_TILE * x) as i32, (TILEMAP_TILE * y) as i32, TILEMAP_TILE, TILEMAP_TILE)
}

fn tab_name(tab: &GameMenuTab) -> String {
    match *tab {
        GameMenuTab::Unit => "Unit".to_string(),
        GameMenuTab::Log => "Log".to_string()
    }
}

fn tab_default_icon(tab: &GameMenuTab) -> u32 {
    match *tab {
        GameMenuTab::Unit => 140,
        GameMenuTab::Log => 9
    }
}


impl GUI<'_> {
    pub fn render(&mut self, state: &mut State) {
        self.canvas.set_draw_color(BG_COLOR.clone());
        self.canvas.clear();

        match self.menu {
            GuiMenu::GameMenu(_) => {
                self.draw_map(state);
                self.draw_unit_list(state);
                self.draw_menu(state);
                self.draw_statusline(state);
            },
            GuiMenu::MainMenu(_) => {
                self.draw_main_menu(state);
            },
            _ => {}
        }

        self.canvas.present();
    }


    fn draw_tile(&mut self, x: u32, y: u32, tile_idx: u32) {
        self.canvas.copy(&self.tileset, tile_rect(tile_idx),
                    Rect::new((x * TILE_SIZE) as i32, (y * TILE_SIZE) as i32, TILE_SIZE, TILE_SIZE)).unwrap();
    }

    fn draw_tile_real_xy(&mut self, x: u32, y: u32, tile_idx: u32) {
        self.canvas.copy(&self.tileset, tile_rect(tile_idx),
                    Rect::new(x as i32, y as i32, TILE_SIZE, TILE_SIZE)).unwrap();
    }

    fn draw_text<T: ToString>(&mut self, x: u32, y: u32, text: T) {
        let string: String = text.to_string();
        for (i, char) in string.chars().enumerate() {
            self.draw_tile(x + i as u32, y, char as u32);
        }
    }

    fn draw_text_real_xy<T: ToString>(&mut self, x: u32, y: u32, text: T) {
        let string: String = text.to_string();
        for (i, char) in string.chars().enumerate() {
            self.draw_tile_real_xy(x + TILE_SIZE * i as u32, y, char as u32);
        }
    }


    fn draw_main_menu(&mut self, state: &mut State) {
        let (width, height) = self.canvas.output_size().unwrap();

        let title = "Necronix";
        let button_text = "Start";
        self.tileset.set_color_mod(255, 255, 255);
        self.draw_text_real_xy(width / 2 - title.len() as u32 * TILE_SIZE / 2, height / 2 - TILE_SIZE / 2 - TILE_SIZE * 2, title);
        self.tileset.set_color_mod(100, 100, 100);
        self.draw_tile_real_xy(width / 2 - TILE_SIZE / 2 - TILE_SIZE * 2, height / 2 + TILE_SIZE / 2 - TILE_SIZE, '<' as u32);
        self.draw_tile_real_xy(width / 2 - TILE_SIZE / 2 + TILE_SIZE * 2, height / 2 + TILE_SIZE / 2 - TILE_SIZE, '>' as u32);
        self.tileset.set_color_mod(100, 0, 200);
        self.draw_tile_real_xy(width / 2 - TILE_SIZE / 2, height / 2 + TILE_SIZE / 2 - TILE_SIZE, 140);
        self.tileset.set_color_mod(100, 0, 200);
        self.draw_text_real_xy(width / 2 - button_text.len() as u32 * TILE_SIZE / 2, height / 2 - TILE_SIZE / 2 + TILE_SIZE * 2, button_text);
    }

    fn draw_map(&mut self, state: &mut State) {
        let map = state.ecs.fetch::<Map>();

        self.canvas.set_draw_color(DARK_BG_COLOR.clone());
        self.canvas.fill_rect(Rect::new(0, 0, TILE_SIZE * MAP_SIZE, TILE_SIZE * MAP_SIZE)).unwrap();
        self.tileset.set_color_mod(100, 100, 100);
        let mut y = 0;
        let mut x = 0;
        for tile in map.tiles.iter() {
            if *tile == 1 {
                self.draw_tile(x, y, 0xdb);
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
                self.tileset.set_color_mod(render.color.0 / 2 * 3, render.color.1 / 2 * 3, render.color.2 / 2 * 3);
            } else {
                self.tileset.set_color_mod(render.color.0, render.color.1, render.color.2);
            }
            self.draw_tile(pos.x, pos.y, render.glyph);
        }
    }

    fn draw_unit_list(&mut self, state: &mut State) {
        let units = state.ecs.read_storage::<Unit>();
        let renderables = state.ecs.read_storage::<Renderable>();
        let map = state.ecs.fetch::<Map>();

        let mut x = 0;
        let mut y = 0;

        for (i, (_unit, render)) in (&units, &renderables).join().enumerate() {
            if i == state.selected_unit_index {
                self.tileset.set_color_mod(render.color.0 / 2 * 3, render.color.1 / 2 * 3, render.color.2 / 2 * 3);
            } else {
                self.tileset.set_color_mod(render.color.0, render.color.1, render.color.2);
            }
            self.draw_tile(x, MAP_SIZE + y, render.glyph);

            x += 1;
            if x == map.width {
                y += 1;
                x = 0;
            }
        }
    }

    fn draw_menu(&mut self, state: &mut State) {
        let (width, height) = self.canvas.output_size().unwrap();

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

        self.canvas.set_draw_color(DARK_BG_COLOR.clone());
        self.canvas.fill_rect(Rect::new((TILE_SIZE * MAP_SIZE) as i32, 0, width - MAP_SIZE * TILE_SIZE, TILE_SIZE)).unwrap();

        for i in 0..GameMenuTabVariants {
            let tab = match i {
                0 => GameMenuTab::Log,
                1 => GameMenuTab::Unit,
                _ => GameMenuTab::Unit
            };

            let name = tab_name(&tab);
            let icon = tab_default_icon(&tab);

            if i == state.gui_game_mode_index {
                self.canvas.set_draw_color(BG_COLOR.clone());
                self.canvas.fill_rect(Rect::new((TILE_SIZE * (MAP_SIZE + i as u32 * 3)) as i32, 0, 3 * TILE_SIZE, TILE_SIZE)).unwrap();
                self.tileset.set_color_mod(200, 200, 200);
            } else {
                self.tileset.set_color_mod(125, 125, 125);
            }

            match tab {
                GameMenuTab::Unit => {
                    if let Some(renderable) = renderable {
                        if i == state.gui_game_mode_index { self.tileset.set_color_mod(renderable.color.0 * 2, renderable.color.1 * 2, renderable.color.2 * 2); }
                        else { self.tileset.set_color_mod(renderable.color.0, renderable.color.1, renderable.color.2); }
                        self.draw_tile(MAP_SIZE + i as u32 * 3 + 1, 0, renderable.glyph);
                    } else {
                        self.draw_tile(MAP_SIZE + i as u32 * 3 + 1, 0, icon);
                    }
                },
                _ => {
                    self.draw_tile(MAP_SIZE + i as u32 * 3 + 1, 0, icon);
                }
            }
        }

        match state.gui_game_mode_index {
            0 => {
                self.draw_log(state, MAP_SIZE, 1);
            },
            1 => {
                self.draw_unit_info(state, MAP_SIZE, 1);
            },
            _ => {}
        }
    }

    fn draw_log(&mut self, state: &mut State, x: u32, y: u32) {
        self.tileset.set_color_mod(100, 200, 200);
        for (i, entry) in state.log.entries.iter().rev().enumerate() {
            self.draw_text(x, y + i as u32, entry);
        }
    }

    fn draw_unit_info(&mut self, state: &mut State, x: u32, y: u32) {
        let units = state.ecs.read_storage::<Unit>();
        let positions = state.ecs.read_storage::<Position>();
        let renderables = state.ecs.read_storage::<Renderable>();
        let entities = state.ecs.entities();
        let names = state.ecs.read_storage::<Name>();

        for (i, (_, entity, position, render)) in (&units, &entities, &positions, &renderables).join().enumerate() {
            if i == state.selected_unit_index {
                self.tileset.set_color_mod(render.color.0 * 2, render.color.1 * 2, render.color.2 * 2);
                let mut name = "Unnamed";
                self.draw_text(x, y, "Unnamed");
                self.tileset.set_color_mod(200, 200, 200);
                self.draw_text(x + name.len() as u32 + 1, y, format!("{}:{}", position.x, position.y));

                break;
            }
        }
    }

    fn draw_statusline(&mut self, state: &mut State) {
        let (width, height) = self.canvas.output_size().unwrap();
        self.canvas.set_draw_color(DARK_BG_COLOR.clone());
        self.canvas.fill_rect(Rect::new(0, (height - TILE_SIZE) as i32, width, TILE_SIZE)).unwrap();


        let tab = match state.gui_game_mode_index {
            0 => GameMenuTab::Log,
            1 => GameMenuTab::Unit,
            _ => GameMenuTab::Unit
        };

        self.tileset.set_color_mod(200, 200, 200);
        self.draw_tile_real_xy(0, height - TILE_SIZE, 7);
        self.draw_text_real_xy(2 * TILE_SIZE, height - TILE_SIZE, tab_name(&tab));
    }

}
