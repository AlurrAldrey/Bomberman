use crate::bomb::Bomb;
use crate::bomb::BombType;

use crate::enemy::Enemy;

use crate::helpers;

#[derive(Clone, Debug)]
pub enum MapObject {
    Enemy(Enemy),
    Bomb(Bomb),
    Rock { id: String },
    Wall { id: String },
    Deviation { id: String, direction: char },
    Nothing { id: String },
}

impl MapObject {
    //Genera un MapObject a partir del string que se le envíe
    //TODO: usar news para achicar la función
    pub fn new(string_rep: &str, position: (u32, u32)) -> Option<Self> {
        match string_rep.chars().nth(0) {
            Some('S') => {
                let range_unsigned =
                    helpers::get_u32_from_char(string_rep.chars().nth(1), string_rep);
                return Some(MapObject::Bomb(Bomb {
                    id: String::from("S"),
                    range: range_unsigned,
                    bomb_type: BombType::Shredding,
                    position,
                }));
            }
            Some('B') => {
                let range_unsigned =
                    helpers::get_u32_from_char(string_rep.chars().nth(1), string_rep);
                return Some(MapObject::Bomb(Bomb {
                    id: String::from("B"),
                    range: range_unsigned,
                    bomb_type: BombType::Normal,
                    position,
                }));
            }
            Some('F') => {
                let health_unsigned =
                    helpers::get_u32_from_char(string_rep.chars().nth(1), string_rep);
                return Some(MapObject::Enemy(Enemy {
                    id: String::from("F"),
                    health: health_unsigned,
                    hitted_by: Vec::new(),
                }));
            }
            Some('D') => {
                let direction_char_option = string_rep.chars().nth(1);
                match direction_char_option {
                    Some(direction) => {
                        return Some(MapObject::Deviation {
                            id: String::from("D"),
                            direction,
                        })
                    } //TODO:Validar direcciones
                    None => {
                        println!("direccion incorrecta en {string_rep}");
                        return None;
                    }
                }
            }
            Some('W') => {
                return Some(MapObject::Wall {
                    id: String::from("W"),
                })
            }
            Some('R') => {
                return Some(MapObject::Rock {
                    id: String::from("R"),
                })
            }
            Some('_') => {
                return Some(MapObject::Nothing {
                    id: String::from("_"),
                })
            }
            _ => None,
        }
    }

    pub fn pretty_print(&self) {
        match self {
            MapObject::Enemy(enemy) => {
                let id = enemy.id.clone();
                let health = enemy.health.clone();
                print!("{id}{health}")
            }
            MapObject::Bomb(bomb) => {
                let id = bomb.id.clone();
                let range = bomb.range.clone();
                print!("{id}{range}")
            }
            MapObject::Deviation { id, direction } => {
                print!("{id}{direction}")
            }
            MapObject::Wall { id } => {
                print!("{id}")
            }
            MapObject::Rock { id } => {
                print!("{id}")
            }
            MapObject::Nothing { id } => {
                print!("{id}")
            }
        }
    }
}
