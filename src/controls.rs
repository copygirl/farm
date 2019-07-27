use std::convert::TryFrom;

use amethyst::{
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::components::{Direction, Movement};

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct ControlsSystem;

impl<'s> System<'s> for ControlsSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Player>,
        WriteStorage<'s, Movement>,
    );

    fn run(&mut self, (
        entities,
        input,
        players,
        mut movements,
    ): Self::SystemData) {
        // TODO: Use events and move dependent on last movement key being pressed.
        let vec = (
            (input.axis_value("horizontal").unwrap() as i32).signum(),
            (input.axis_value("vertical").unwrap() as i32).signum(),
        );

        match Direction::try_from(vec) {
            Ok(dir) => {
                for (entity, _) in (&*entities, &players).join() {
                    movements.entry(entity).unwrap().or_insert_with(|| {
                        Movement::new(dir)
                    });
                }
            },
            _ => {}
        }
        
    }
}
