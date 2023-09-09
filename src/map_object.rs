#[derive(Debug)]
pub enum MapObject {
    Enemy{id: String, health: i32},
    NormalBomb{id: String, range: i32}, //normal y shredding podrian ser el mismo tipo?
    ShreddingBomb{id: String, range:i32},
    Rock{id: String},
    Wall{id: String},
    Deviation{id: String, direction: char}
}

impl MapObject{
    pub fn new(string_rep: &str)-> Option<Self> {
        match string_rep.chars().nth(0) {
            Some('W') => {
                return Some(MapObject::Wall { id: String::from("W") })
            },
            _ => None,
        }
    }
}