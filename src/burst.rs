use crate::matrix::map_object::MapObject;

#[derive(Debug)]
pub struct Burst {
    pub direction: char,
    pub starting_position: (usize,usize),
    pub range: u32,
    pub bomb: MapObject
}

impl Burst {
    pub fn new(direction: char, starting_position: (usize, usize), range: u32, bomb: MapObject) -> Self {
        Burst {
            direction,
            starting_position,
            range,
            bomb
        }
    }
}