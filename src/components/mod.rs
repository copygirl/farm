use std::convert::TryFrom;

mod position;
mod movement;

pub use position::Position;
pub use movement::{Movement, Speed};


#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum Direction { Up, Down, Left, Right }

impl TryFrom<(i32, i32)> for Direction {
    type Error = ();

    fn try_from(vec: (i32, i32)) -> Result<Self, Self::Error> {
        match vec {
            (-1,  _) => Ok(Direction::Left),
            ( 1,  _) => Ok(Direction::Right),
            ( _, -1) => Ok(Direction::Up),
            ( _,  1) => Ok(Direction::Down),
            _ => Err(())
        }
    }
}
