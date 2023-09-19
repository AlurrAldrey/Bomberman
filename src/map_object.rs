use crate::bomb::Bomb;
use crate::bomb::BombType;

use crate::enemy::Enemy;

use crate::helpers;

///Representa un objeto genérico del mapa
#[derive(Clone, Debug, PartialEq)]
pub enum MapObject {
    Enemy(Enemy),
    Bomb(Bomb),
    Rock { id: String },
    Wall { id: String },
    Deviation { id: String, direction: char },
    Nothing { id: String },
}

impl MapObject {
    ///Genera un MapObject a partir del string que se le envíe
    pub fn new(string_rep: &str, position: (u32, u32)) -> Option<Self> {
        match string_rep.chars().nth(0) {
            Some('S') => Self::create_bomb('S', string_rep, position),
            Some('B') => Self::create_bomb('B', string_rep, position),
            Some('F') => Self::create_enemy(string_rep),
            Some('D') => {
                let direction_char_option = string_rep.chars().nth(1);
                direction_char_option.map(|direction| MapObject::Deviation {
                    id: String::from("D"),
                    direction,
                })
            }
            Some('W') => Some(MapObject::Wall {
                id: String::from("W"),
            }),
            Some('R') => Some(MapObject::Rock {
                id: String::from("R"),
            }),
            Some('_') => Some(MapObject::Nothing {
                id: String::from("_"),
            }),
            _ => None,
        }
    }

    ///muestra al objeto representado con un string
    pub fn show(&self) -> String {
        match self {
            MapObject::Enemy(enemy) => {
                let id = enemy.id.clone();
                let health = enemy.health;
                format!("{}{}", id, health)
            }
            MapObject::Bomb(bomb) => {
                let id = bomb.id.clone();
                let range = bomb.range;
                format!("{}{}", id, range)
            }
            MapObject::Deviation { id, direction } => {
                format!("{}{}", id, direction)
            }
            MapObject::Wall { id } => id.to_string(),
            MapObject::Rock { id } => id.to_string(),
            MapObject::Nothing { id } => id.to_string(),
        }
    }

    ///crea una variante Bomb de MapObject
    fn create_bomb(id: char, string_rep: &str, position: (u32, u32)) -> Option<MapObject> {
        let range_unsigned;
        match helpers::get_u32_from_char(string_rep.chars().nth(1)) {
            Some(n) => range_unsigned = n,
            None => return None,
        }

        Some(MapObject::Bomb(Bomb {
            id: String::from(id),
            range: range_unsigned,
            bomb_type: BombType::new(id),
            position,
        }))
    }

    ///Crea una variante Enemy de MapObject
    fn create_enemy(string_rep: &str) -> Option<MapObject> {
        let health_unsigned;
        match helpers::get_u32_from_char(string_rep.chars().nth(1)) {
            Some(n) => health_unsigned = n,
            None => return None,
        }
        Some(MapObject::Enemy(Enemy {
            id: String::from("F"),
            health: health_unsigned,
            hitted_by: Vec::new(),
        }))
    }

    // DEBUG
    // pub fn pretty_print(&self) { //para debug
    //     match self {
    //         MapObject::Enemy(enemy) => {
    //             let id = enemy.id.clone();
    //             let health = enemy.health.clone();
    //             print!("{id}{health}")
    //         }
    //         MapObject::Bomb(bomb) => {
    //             let id = bomb.id.clone();
    //             let range = bomb.range.clone();
    //             print!("{id}{range}")
    //         }
    //         MapObject::Deviation { id, direction } => {
    //             print!("{id}{direction}")
    //         }
    //         MapObject::Wall { id } => {
    //             print!("{id}")
    //         }
    //         MapObject::Rock { id } => {
    //             print!("{id}")
    //         }
    //         MapObject::Nothing { id } => {
    //             print!("{id}")
    //         }
    //     }
    // }
}
