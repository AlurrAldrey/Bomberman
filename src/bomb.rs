#[derive(Clone, Debug, PartialEq)]
pub struct Bomb {
    pub id: String,
    pub range: u32,
    pub bomb_type: BombType,
    pub position: (u32, u32),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BombType {
    Normal,
    Shredding,
}

impl Bomb {
    pub fn new(position: (u32, u32)) -> Bomb {
        return Bomb {
            id: String::from('*'),
            range: 0,
            bomb_type: BombType::Normal,
            position
        };
    }
}
