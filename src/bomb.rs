#[derive(Clone, Debug)]
pub struct Bomb {
    pub id: String,
    pub range: u32,
    pub bomb_type: BombType,
    pub position: (u32, u32),
}

#[derive(Clone, Debug)]
pub enum BombType {
    Normal,
    Shredding,
}
