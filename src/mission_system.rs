use specs::prelude::*;
use super::{Unit, Position, Mission};

pub struct MissionSystem {}

impl<'a> System<'a> for MissionSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Unit>,
        WriteStorage<'a, Position>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_entities, mut units, mut positions) = data;

        for (unit, pos) in (&mut units, &mut positions).join()  {
            match unit.mission {
                Mission::GoTo(x, y) => {
                    let dx = (x as i32 - pos.x as i32) as f32;
                    let dy = (y as i32 - pos.y as i32) as f32;

                    if dx == 0. && dy == 0. {
                        unit.mission = Mission::Stay;
                    }

                    let length = (dx * dx + dy * dy).sqrt();

                    let nx = (dx / length).round() as i32;
                    let ny = (dy / length).round() as i32;

                    pos.x = (pos.x as i32 + nx) as u32;
                    pos.y = (pos.y as i32 + ny) as u32;
                },
                _ => {}
            }
        }
    }
}
