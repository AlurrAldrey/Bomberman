pub mod map_object;
use self::map_object::MapObject;
use self::map_object::Enemy;
use self::map_object::Bomb;

use crate::burst::Burst;
use crate::helpers::u32_to_usize; 


#[derive(Debug)]
pub struct Matrix {
    pub dimension: u32,
    data: Vec<Vec<MapObject>>,
}

pub enum AffectResponse{
    Deviate{ direction: char }, 
    Explode { bomb: Bomb}, 
    Stop, 
    Continue,
    AffectError{ err: String }
}

impl Matrix{

    pub fn new(input: String)-> Self {
        let mut data: Vec<Vec<MapObject>> = Vec::new();
        let lines = input.split("\r\n")
                                                        .filter(|&x| !x.is_empty()); //sacar la linea vacia despues del ultimo salto de linea
        let dimension = lines.clone().count() as u32;

        let mut i = 0;
        for line in lines {

            let elements = line.split(" ");
            let row_dimension = elements.clone().count() as u32;
            if row_dimension != dimension { 
                //ver que hacer para resetear el programa
                println!("La matriz debe ser cuadrada, (chequea no tener un salto de linea de más)");
            }

            let mut row: Vec<MapObject> = Vec::new();
            let mut j = 0;
            for element in elements{
                let map_object_result = map_object::MapObject::new(&element, (j,i));
                match map_object_result {
                    Some(map_object) => {row.push(map_object);},
                    None => {println!("hubo error al crear el objeto")} //TODO: devolver None
                }
                j += 1;
            }
            data.push(row);
            i += 1;
        }
        Self {
            dimension, data
        }
    }

    pub fn set(&mut self, row: u32, col: u32, value: MapObject) {
        if row > self.dimension || col > self.dimension { 
            print!("ERROR!: se intento acceder a un indice mayor a la dimansion de la matriz");
            return
        }
        let row = u32_to_usize(row);
        let col = u32_to_usize(col);
        self.data[col][row] = value;
    }

    pub fn get(&self, row: u32, col: u32) -> MapObject {
        let row = u32_to_usize(row);
        let col = u32_to_usize(col);
        return self.data[col][row].clone();
    }

    pub fn pretty_print(&self) {
        for i in 0..(self.dimension) {
            for j in 0..(self.dimension) {
                let object = self.get(j,i);//van al revez para que se imprima como piden
                object.pretty_print();
                if j != self.dimension -1 {print!(" ");}
            }
            print!("\r\n");
        }
    }

    pub fn affect_position(& mut self, position_to_affect: (u32,u32), current_burst: &Burst) -> AffectResponse {
        let object_affected = self.get(position_to_affect.0.clone(), position_to_affect.1.clone());
        match object_affected {
            MapObject::Nothing { id } => { return AffectResponse::Continue },
            MapObject::Wall{id} =>{ return AffectResponse::Stop },
            MapObject::Enemy(enemy) => { return self.damage_enemy(enemy, position_to_affect, &current_burst.bomb);},
            MapObject::Rock { id } => {
                match current_burst.bomb.bomb_type {
                    map_object::BombType::Shredding => { return AffectResponse::Continue },
                    map_object::BombType::Normal=> { return AffectResponse::Stop }
                }
            },
            MapObject::Deviation { id, direction } => { return AffectResponse::Deviate { direction }},
            MapObject::Bomb (bomb) => {/* Explode(bomba), la reemplazo en la matriz por un Nothing */
                self.set(position_to_affect.0, position_to_affect.1, MapObject::Nothing { id: String::from('_') });
                return AffectResponse::Explode{ bomb };
            }
            _ => { todo!()}
        }
    }

    fn damage_enemy(& mut self, mut enemy: Enemy, position_to_affect: (u32,u32), bomb: &Bomb) -> AffectResponse {
        let is_killed = enemy.damage(bomb);
            match is_killed {
                Some(is_killed) => {
                    if is_killed { 
                        self.set(position_to_affect.0, position_to_affect.1, MapObject::Nothing { id: String::from('_')})
                    } else {
                        self.set(position_to_affect.0, position_to_affect.1, MapObject::Enemy(enemy));
                    }
                    return AffectResponse::Continue
                },
                None => { AffectResponse::AffectError{ err: String::from("se intentó atacar a un muerto")}}
            }
    }
}