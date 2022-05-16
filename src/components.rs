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

pub enum MaterialType { Logs }

#[derive(Component)]
pub struct Material {
    pub material_type: MaterialType
}

#[derive(Component)]
pub struct Choppable {
    pub chops_into: MaterialType
}

pub enum Mission { Stay, GoTo(u32, u32), Chop(Entity) }

impl Mission {
    pub fn get_description(&self) -> String {
        match self {
            Mission::Stay => "Stay".to_string(),
            Mission::GoTo(x, y) => format!("GoTo {}:{}", x, y).to_string(),
            Mission::Chop(entity) => "Chop".to_string()
        }
    }
}

#[derive(Component)]
pub struct Unit {
    pub mission: Mission
}

#[derive(Component)]
pub struct Living {
    max_health: i32,
    health: i32
}
