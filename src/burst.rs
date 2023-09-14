use crate::matrix::map_object::Bomb;

#[derive(Debug)]
pub struct Burst {
    pub direction: char,
    pub starting_position: (u32,u32),
    pub range: u32,
    pub bomb: Bomb
}

impl Burst {
    pub fn new(direction: char, starting_position: (u32, u32), range: u32, bomb: Bomb) -> Self {
        Burst {
            direction,
            starting_position,
            range,
            bomb
        }
    }
}