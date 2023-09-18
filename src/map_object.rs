use crate::bomb::Bomb;
use crate::bomb::BombType;

use crate::enemy::Enemy;

use crate::helpers;

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
    //Genera un MapObject a partir del string que se le envíe
    pub fn new(string_rep: &str, position: (u32, u32)) -> Option<Self> {
        match string_rep.chars().nth(0) {
            Some('S') => {
                Self::create_bomb('S',string_rep,position)
            }
            Some('B') => {
                Self::create_bomb('B',string_rep,position)
            }
            Some('F') => {
                Self::create_enemy(string_rep)
            }
            Some('D') => {
                let direction_char_option = string_rep.chars().nth(1);
                match direction_char_option {
                    Some(direction) => {
                        return Some(MapObject::Deviation {
                            id: String::from("D"),
                            direction,
                        })
                    }
                    None => {
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

    pub fn to_string(&self) -> String{
        match self {
            MapObject::Enemy(enemy) => {
                let id = enemy.id.clone();
                let health = enemy.health.clone();
                return format!("{}{}", id, health);
            }
            MapObject::Bomb(bomb) => {
                let id = bomb.id.clone();
                let range = bomb.range.clone();
                return format!("{}{}", id, range);
            }
            MapObject::Deviation { id, direction } => {
                return format!("{}{}", id, direction);
            }
            MapObject::Wall { id } => {
                return format!("{}", id);
            }
            MapObject::Rock { id } => {
                return format!("{}", id);
            }
            MapObject::Nothing { id } => {
                return format!("{}", id);
            }
        }
    }

    fn create_bomb(id: char, string_rep: &str, position: (u32, u32)) -> Option<MapObject>{
        let range_unsigned ;
        match helpers::get_u32_from_char(string_rep.chars().nth(1), string_rep) {
            Some(n) => { range_unsigned = n },
            None => { return None }
        }
        
        let b_type: BombType;
        if id == 'S' {
            b_type = BombType::Shredding;
        } else {
            b_type = BombType::Normal;
        }

        return Some(MapObject::Bomb(Bomb {
            id: String::from(id),
            range: range_unsigned,
            bomb_type: b_type,
            position,
        }));
    }

    fn create_enemy(string_rep: &str) -> Option<MapObject> {
        let health_unsigned ;
        match helpers::get_u32_from_char(string_rep.chars().nth(1), string_rep) {
            Some(n) => { health_unsigned = n },
            None => { return None }
        }
        return Some(MapObject::Enemy(Enemy {
            id: String::from("F"),
            health: health_unsigned,
            hitted_by: Vec::new(),
        }));
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
