use amethyst::ecs::prelude::*;

use crate::components::Direction;


pub struct Movement {
    pub direction: Direction,
    pub progress: f32,
}

impl Movement {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            progress: 0.0,
        }
    }
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Speed(pub f32);

impl Component for Speed {
    type Storage = DenseVecStorage<Self>;
}
