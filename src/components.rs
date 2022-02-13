use specs::prelude::*;
use specs::Component;

#[derive(Component)]
pub struct Renderable {
    pub glyph: u32,
    pub color: (u8, u8, u8)
}

#[derive(Component)]
pub struct Position {
    pub x: u32,
    pub y: u32
}

#[derive(Component)]
pub struct Unit {}
