///represneta una bomba
#[derive(Clone, Debug, PartialEq)]
pub struct Bomb {
    pub id: String,
    pub range: u32,
    pub bomb_type: BombType,
    pub position: (u32, u32),
}

///representa  el tipo de bomba
#[derive(Clone, Debug, PartialEq)]
pub enum BombType {
    Normal,
    Shredding,
}

impl Bomb {
    ///crea una bomba genérica
    pub fn new(position: (u32, u32)) -> Bomb {
        Bomb {
            id: String::from('*'),
            range: 0,
            bomb_type: BombType::Normal,
            position,
        }
    }
}

impl BombType {
    ///crea una variante de BombType de acuerdo al caracter del id
    pub fn new(type_char: char) -> BombType {
        match type_char {
            'S' => BombType::Shredding,
            _ => BombType::Normal,
        }
    }
}
