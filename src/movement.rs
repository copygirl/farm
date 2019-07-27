use amethyst::{
    core::{transform::Transform, Time},
    ecs::prelude::*,
};


use crate::components::{Position, Movement, Speed};

const CELL_SIZE: i32 = 16;
const DEFAULT_SPEED: f32 = 4.0;

#[derive(Default)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        WriteStorage<'s, Position>,
        WriteStorage<'s, Movement>,
        WriteStorage<'s, Speed>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (
        entities,
        time,
        mut positions,
        mut movements,
        speeds,
        mut transforms,
    ): Self::SystemData) {
        let mut to_remove = Vec::new();
        for (entity, pos, mut movement, transform)
        in (&*entities, &mut positions, &mut movements, &mut transforms).join() {
            let move_vec = Position::from(movement.direction);
            if movement.progress == 0.0 { *pos += move_vec; }

            movement.progress += time.delta_seconds()
                * speeds.get(entity).map_or(DEFAULT_SPEED, |speed| speed.0);
            if movement.progress >= 1.0 {
                movement.progress = 1.0;
                to_remove.push(entity);
            }

            transform.set_translation_xyz(
                (pos.x as f32 - (move_vec.x as f32 * (1.0 - movement.progress))) * CELL_SIZE as f32,
                (pos.y as f32 - (move_vec.y as f32 * (1.0 - movement.progress))) * CELL_SIZE as f32,
                0.0
            );
        }
        for entity in to_remove {
            movements.remove(entity);
        }
    }
}
