use amethyst::{
    ecs::prelude::*,
    input::{InputHandler, StringBindings},
};

use crate::movement::{Facing, Movement};

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
        WriteStorage<'s, Facing>,
        WriteStorage<'s, Movement>,
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Player>,
    );

    fn run(&mut self, (
        entities,
        mut facings,
        mut movements,
        input,
        players
    ): Self::SystemData) {
        // TODO: Use events and move dependent on last movement key being pressed.
        let vec = (
            (input.axis_value("horizontal").unwrap() as i32).signum(),
            (input.axis_value("vertical").unwrap() as i32).signum(),
        );

        match Facing::from_vec(vec) {
            Some(facing) => {
                for (entity, _) in (&*entities, &players).join() {
                    facings.insert(entity, facing.clone()).unwrap();
                    movements.entry(entity).unwrap()
                        .or_insert_with(|| Movement::new(facing));
                }
            },
            _ => {}
        }
        
    }
}
