#[derive(Debug)]
pub struct Burst {
    pub direction: char,
    pub starting_position: (usize,usize),
    pub range: u32
}

impl Burst {
    pub fn new(direction: char, starting_position: (usize, usize), range: u32) -> Self {
        Burst {
            direction,
            starting_position,
            range,
        }
    }
}