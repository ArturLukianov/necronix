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
pub enum GuiMenu { MainMenu(MainMenuButton), HelpMenu, GameMenu(GameMenuTab), CreditsMenu }

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MainMenuButton { Start, Help, Credits }

impl MainMenuButton {
    pub fn variants_count() -> u32 { 3 }
    pub fn from_u8(id: u8) -> MainMenuButton {
        match id {
            0 => MainMenuButton::Start,
            1 => MainMenuButton::Help,
            2 => MainMenuButton::Credits,
            _ => MainMenuButton::Start
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            MainMenuButton::Start => 0,
            MainMenuButton::Help => 1,
            MainMenuButton::Credits => 2
        }
    }

    pub fn next(&self) -> MainMenuButton {
        MainMenuButton::from_u8((self.to_u8() + 1) % MainMenuButton::variants_count() as u8)
    }

    pub fn prev(&self) -> MainMenuButton {
        let i = self.to_u8();
        if i == 0 {
            return MainMenuButton::from_u8(MainMenuButton::variants_count() as u8 - 1);
        }
        MainMenuButton::from_u8((self.to_u8() - 1) % MainMenuButton::variants_count() as u8)
    }

    pub fn get_text(&self) -> String {
        match self {
            MainMenuButton::Start => "Start".to_string(),
            MainMenuButton::Help => "Help".to_string(),
            MainMenuButton::Credits => "Credits".to_string()
        }
    }

    pub fn get_icon(&self) -> u32 {
        match self {
            MainMenuButton::Start => 140,
            MainMenuButton::Help => '?' as u32,
            MainMenuButton::Credits => '@' as u32
        }
    }

    pub fn get_color(&self) -> (u8, u8, u8) {
        match self {
            MainMenuButton::Start => (100, 0, 200),
            MainMenuButton::Help => (200, 100, 0),
            MainMenuButton::Credits => (200, 0, 100)
        }
    }

    pub fn get_menu(&self) -> GuiMenu {
        match self {
            MainMenuButton::Start => GuiMenu::GameMenu(GameMenuTab::Unit),
            MainMenuButton::Help => GuiMenu::HelpMenu,
            MainMenuButton::Credits => GuiMenu::CreditsMenu
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum GameMenuTab { Unit, Log }

impl GameMenuTab {
    pub fn variants_count() -> u32 { 2 }
    pub fn from_u8(id: u8) -> GameMenuTab {
        match id {
            0 => GameMenuTab::Unit,
            1 => GameMenuTab::Log,
            _ => GameMenuTab::Unit
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            GameMenuTab::Unit => 0,
            GameMenuTab::Log => 1
        }
    }

    pub fn next(&self) -> GameMenuTab {
        GameMenuTab::from_u8((self.to_u8() + 1) % GameMenuTab::variants_count() as u8)
    }
}

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
            GuiMenu::GameMenu(tab) => {
                self.draw_map(state);
                self.draw_unit_list(state);
                self.draw_menu(state, tab);
                self.draw_statusline(state, tab);
            },
            GuiMenu::MainMenu(_) => {
                self.draw_main_menu(state);
            },
            GuiMenu::HelpMenu => {
                self.draw_help_menu();
            },
            GuiMenu::CreditsMenu => {
                self.draw_credits_menu();
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


    fn draw_help_menu(&mut self) {
        let (width, height) = self.canvas.output_size().unwrap();

        let text = "Help";
        self.tileset.set_color_mod(200, 100, 0);
        self.draw_text_real_xy(width / 2 - text.len() as u32 * TILE_SIZE / 2, TILE_SIZE / 2, text);

        self.tileset.set_color_mod(200, 200, 200);
        for i in 0..20 {
            let text = "You should take a look into README.md";
            self.draw_text_real_xy(width / 2 - text.len() as u32 * TILE_SIZE / 2, TILE_SIZE / 2 + TILE_SIZE * (i + 2), text);
        }
    }


    fn draw_credits_menu(&mut self) {
        let (width, height) = self.canvas.output_size().unwrap();
        let text = "Author: ArturLukianov";
        self.tileset.set_color_mod(200, 200, 200);
        self.draw_text_real_xy(width / 2 - text.len() as u32 * TILE_SIZE / 2, height / 2 - TILE_SIZE / 2, "Author:");
        self.tileset.set_color_mod(200, 0, 100);
        self.draw_text_real_xy(width / 2 - text.len() as u32 * TILE_SIZE / 2 + TILE_SIZE * 8, height / 2 - TILE_SIZE / 2, "ArturLukianov");
    }


    fn draw_main_menu(&mut self, state: &mut State) {
        let (width, height) = self.canvas.output_size().unwrap();

        let title = "Necronix";

        match self.menu {
            GuiMenu::MainMenu(button) => {
                let button_text = button.get_text();
                let button_color = button.get_color();

                self.tileset.set_color_mod(255, 255, 255);
                self.draw_text_real_xy(width / 2 - title.len() as u32 * TILE_SIZE / 2, height / 2 - TILE_SIZE / 2 - TILE_SIZE * 2, title);
                self.tileset.set_color_mod(100, 100, 100);
                self.draw_tile_real_xy(width / 2 - TILE_SIZE / 2 - TILE_SIZE * 2, height / 2 + TILE_SIZE / 2 - TILE_SIZE, '<' as u32);
                self.draw_tile_real_xy(width / 2 - TILE_SIZE / 2 + TILE_SIZE * 2, height / 2 + TILE_SIZE / 2 - TILE_SIZE, '>' as u32);
                self.tileset.set_color_mod(button_color.0, button_color.1, button_color.2);
                self.draw_tile_real_xy(width / 2 - TILE_SIZE / 2, height / 2 + TILE_SIZE / 2 - TILE_SIZE, button.get_icon());
                self.draw_text_real_xy(width / 2 - button_text.len() as u32 * TILE_SIZE / 2, height / 2 - TILE_SIZE / 2 + TILE_SIZE * 2, button_text);
            },
            _ => {return;}
        }

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

    fn draw_menu(&mut self, state: &mut State, current_tab: GameMenuTab) {
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

        for i in 0..GameMenuTab::variants_count() {
            let tab = GameMenuTab::from_u8(i as u8);

            let name = tab_name(&tab);
            let icon = tab_default_icon(&tab);

            if tab == current_tab {
                self.canvas.set_draw_color(BG_COLOR.clone());
                self.canvas.fill_rect(Rect::new((TILE_SIZE * (MAP_SIZE + i as u32 * 3)) as i32, 0, 3 * TILE_SIZE, TILE_SIZE)).unwrap();
                self.tileset.set_color_mod(200, 200, 200);
            } else {
                self.tileset.set_color_mod(125, 125, 125);
            }

            match tab {
                GameMenuTab::Unit => {
                    if let Some(renderable) = renderable {
                        if tab == current_tab { self.tileset.set_color_mod(renderable.color.0 * 2, renderable.color.1 * 2, renderable.color.2 * 2); }
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

        match current_tab {
            GameMenuTab::Log => {
                self.draw_log(state, MAP_SIZE, 1);
            },
            GameMenuTab::Unit => {
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

        for (i, (unit, entity, position, render)) in (&units, &entities, &positions, &renderables).join().enumerate() {
            if i == state.selected_unit_index {
                self.tileset.set_color_mod(render.color.0 * 2, render.color.1 * 2, render.color.2 * 2);
                let mut name = "Unnamed";
                self.draw_text(x, y, "Unnamed");
                self.tileset.set_color_mod(200, 200, 200);
                self.draw_text(x + name.len() as u32 + 1, y, format!("{}:{}", position.x, position.y));
                self.tileset.set_color_mod(150, 150, 150);
                self.draw_text(x, y + 1, unit.mission.get_description());

                break;
            }
        }
    }

    fn draw_statusline(&mut self, state: &mut State, current_tab: GameMenuTab) {
        let (width, height) = self.canvas.output_size().unwrap();
        self.canvas.set_draw_color(DARK_BG_COLOR.clone());
        self.canvas.fill_rect(Rect::new(0, (height - TILE_SIZE) as i32, width, TILE_SIZE)).unwrap();

        self.tileset.set_color_mod(200, 200, 200);
        self.draw_tile_real_xy(0, height - TILE_SIZE, 7);
        self.draw_text_real_xy(2 * TILE_SIZE, height - TILE_SIZE, tab_name(&current_tab));
    }

}
