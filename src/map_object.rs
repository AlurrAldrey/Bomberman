#[derive(Debug)]
pub enum MapObject {
    Enemy{id: String, health: u32},
    NormalBomb{id: String, range: u32}, //normal y shredding podrian ser el mismo tipo?
    ShreddingBomb{id: String, range:u32},
    Rock{id: String},
    Wall{id: String},
    Deviation{id: String, direction: char}
}

impl MapObject{
    pub fn new(string_rep: &str)-> Option<Self> {
        match string_rep.chars().nth(0) {
            Some('F') => {
                let health_char = string_rep.chars().nth(1);
                let health_unsigned = get_u32_from_char(health_char, string_rep);
                return Some(MapObject::Enemy { id: String::from("F"), health: health_unsigned })
            },
            Some('W') => {
                return Some(MapObject::Wall { id: String::from("W") })
            },
            _ => None,
        }
    }
}

fn get_u32_from_char(health_char: Option<char>, string_rep: &str) -> u32 {
    let mut health_unsigned: u32 = 0;
    match health_char {
        Some(health) => { 
            let health_todigit = health.to_digit(10);
            match health_todigit {
                Some(digit) => { health_unsigned = digit },
                None => { println!("El valor de {string_rep} no es un numero") }
            }
        },
        None => { println!("No se encontró el valor de {string_rep}") }
    };
    health_unsigned
}