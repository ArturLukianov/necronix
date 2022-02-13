use specs::prelude::*;
use specs::Component;

#[derive(Component, Clone, Copy)]
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
pub struct Name {
    pub name: String
}

#[derive(Component)]
pub struct Physical {
    pub weight: i32,
    pub size: i32
}

#[derive(Component)]
pub struct BlocksTile {}

#[derive(Component)]
pub struct Unit {}

#[derive(Component)]
pub struct Living {
    max_health: i32,
    health: i32
}
