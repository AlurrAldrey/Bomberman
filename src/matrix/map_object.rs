#[derive(Debug)]
pub enum MapObject {
    Enemy{id: String, health: u32},
    NormalBomb{id: String, range: u32}, //normal y shredding podrian ser el mismo tipo?
    ShreddingBomb{id: String, range:u32},
    Rock{id: String},
    Wall{id: String},
    Deviation{id: String, direction: char},
    Nothing{id: String}
}

impl MapObject{

    //Genera un MapObject a partir del string que se le envíe
    pub fn new(string_rep: &str)-> Option<Self> {
        match string_rep.chars().nth(0) {
            Some('S') => {
                let range_char = string_rep.chars().nth(1);
                let range_unsigned = get_u32_from_char(range_char, string_rep);
                return Some(MapObject::ShreddingBomb { id: String::from("S"), range: range_unsigned })
            },
            Some('B') => {
                let range_char = string_rep.chars().nth(1);
                let range_unsigned = get_u32_from_char(range_char, string_rep);
                return Some(MapObject::NormalBomb{ id: String::from("B"), range: range_unsigned })
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
            MapObject::NormalBomb { id, range } => { print!("{id}{range}") },
            MapObject::ShreddingBomb { id, range } => { print!("{id}{range}") },
            MapObject::Deviation { id, direction } => { print!("{id}{direction}") },
            MapObject::Wall { id} => { print!("{id}") },
            MapObject::Rock { id} => { print!("{id}") },
            MapObject::Nothing { id} => { print!("{id}") },

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
    number_unsigned //TODO: devolver None en caso de error
}