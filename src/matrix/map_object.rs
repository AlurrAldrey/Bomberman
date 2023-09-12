#[derive(Clone)]
#[derive(Debug)]
pub enum MapObject {
    Enemy{id: String, health: u32},
    Bomb{id: String, range: u32, bomb_type: BombType, position: (u32,u32)}, //normal y shredding podrian ser el mismo tipo?
    Rock{id: String},
    Wall{id: String},
    Deviation{id: String, direction: char},
    Nothing{id: String}
}

#[derive(Clone)]
#[derive(Debug)]
pub enum BombType {
    Normal,
    Shredding
}

impl MapObject{

    //Genera un MapObject a partir del string que se le envíe
    pub fn new(string_rep: &str, position: (u32,u32))-> Option<Self> {
        match string_rep.chars().nth(0) {
            Some('S') => {
                let range_char = string_rep.chars().nth(1);
                let range_unsigned = get_u32_from_char(range_char, string_rep);
                return Some(MapObject::Bomb { id: String::from("S"), range: range_unsigned, bomb_type: BombType::Shredding, position })
            },
            Some('B') => {
                let range_char = string_rep.chars().nth(1);
                let range_unsigned = get_u32_from_char(range_char, string_rep);
                return Some(MapObject::Bomb{ id: String::from("B"), range: range_unsigned, bomb_type: BombType::Normal, position })
            },
            Some('F') => {
                let health_char = string_rep.chars().nth(1);
                let health_unsigned = get_u32_from_char(health_char, string_rep);
                return Some(MapObject::Enemy { id: String::from("F"), health: health_unsigned })
            },
            Some('D') => {
                let direction_char_option = string_rep.chars().nth(1);
                let direction_char: char;
                match direction_char_option {
                    Some(direction)=> { direction_char = direction;}, //TODO:Validar direcciones
                    None => {println!("direccion incorrecta en {string_rep}"); return None;}
                }
                return Some(MapObject::Deviation { id: String::from("D"), direction: direction_char })
            },
            Some('W') => {
                return Some(MapObject::Wall { id: String::from("W") })
            },
            Some('R') => {
                return Some(MapObject::Rock { id: String::from("R") })
            },
            Some('_') => {
                return Some(MapObject::Nothing { id: String::from("_") })
            },
            _ => None,
        }
    }

    pub fn pretty_print(&self){
        match self {
            MapObject::Enemy { id, health } => { print!("{id}{health}") },
            MapObject::Bomb { id, range , bomb_type, position} => { print!("{id}{range}") },
            MapObject::Deviation { id, direction } => { print!("{id}{direction}") },
            MapObject::Wall { id} => { print!("{id}") },
            MapObject::Rock { id} => { print!("{id}") },
            MapObject::Nothing { id} => { print!("{id}") },
        }
    }

    pub fn damage(&self) {
        match self {
            MapObject::Enemy { id, health } => { 
                self 
            }
        }
    }
}

fn get_u32_from_char(number_char: Option<char>, string_rep: &str) -> u32 {
    let mut number_unsigned: u32 = 0;
    match number_char {
        Some(health) => { 
            let health_todigit = health.to_digit(10);
            match health_todigit {
                Some(digit) => { number_unsigned = digit },
                None => { println!("El valor de {string_rep} no es un numero") }
            }
        },
        None => { println!("No se encontró el valor de {string_rep}") }
    };
    number_unsigned
}