use amethyst::ecs::prelude::*;

use crate::components::Direction;

#[derive(Eq, PartialEq, Hash, Debug, Default, Copy, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self { Self { x, y } }
}

impl From<(i32, i32)> for Position {
    fn from((x, y): (i32, i32)) -> Self { Self { x, y } }
}

impl From<Direction> for Position {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up    => Self::new( 0, -1),
            Direction::Down  => Self::new( 0,  1),
            Direction::Left  => Self::new(-1,  0),
            Direction::Right => Self::new( 1,  0),
        }
    }
}

impl std::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl std::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) { *self = *self + rhs; }
}

impl std::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl std::ops::SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) { *self = *self - rhs; }
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}
