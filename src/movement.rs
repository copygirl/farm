use amethyst::{
    core::{transform::Transform, Time},
    ecs::prelude::*,
};


const CELL_SIZE: i32 = 16;
const DEFAULT_SPEED: f32 = 4.0;

#[derive(Default)]
pub struct CellPos(pub i32, pub i32);

#[derive(Clone, Copy, Debug)]
pub enum Facing { Up, Down, Left, Right }

pub struct Movement {
    pub direction: Facing,
    progress: f32,
}

pub struct Speed(pub f32);


impl Facing {
    pub fn to_vec(&self) -> (i32, i32) {
        match self {
            Facing::Up => (0, -1),
            Facing::Down => (0, 1),
            Facing::Left => (-1, 0),
            Facing::Right => (1, 0),
        }
    }

    pub fn from_vec(vec: (i32, i32)) -> Option<Self> {
        match vec {
            (-1, _) => Some(Facing::Left),
            (1, _) => Some(Facing::Right),
            (_, -1) => Some(Facing::Up),
            (_, 1) => Some(Facing::Down),
            _ => Option::None
        }
    }
}

impl Movement {
    pub fn new(direction: Facing) -> Self {
        Movement {
            direction,
            progress: 0.0,
        }
    }
}


impl Component for CellPos {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Facing {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Movement {
    type Storage = DenseVecStorage<Self>;
}

impl Component for Speed {
    type Storage = DenseVecStorage<Self>;
}


#[derive(Default)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        Entities<'s>,
        Read<'s, Time>,
        WriteStorage<'s, CellPos>,
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
        for (entity, mut pos, mut movement, transform)
        in (&*entities, &mut positions, &mut movements, &mut transforms).join() {
            let (x, y) = movement.direction.to_vec();
            if movement.progress == 0.0 {
                pos.0 += x;
                pos.1 += y;
            }

            movement.progress += time.delta_seconds()
                * speeds.get(entity).map_or(DEFAULT_SPEED, |speed| speed.0);
            if movement.progress >= 1.0 {
                movement.progress = 1.0;
                to_remove.push(entity);
            }

            transform.set_translation_xyz(
                (pos.0 as f32 - (x as f32 * (1.0 - movement.progress))) * CELL_SIZE as f32,
                (pos.1 as f32 - (y as f32 * (1.0 - movement.progress))) * CELL_SIZE as f32,
                0.0
            );
        }
        for entity in to_remove {
            movements.remove(entity);
        }
    }
}
