use crate::map_object;
use map_object::MapObject;

use crate::bomb::BombType;
use crate::Bomb;

use crate::burst::Burst;
use crate::helpers::u32_to_usize;

use crate::enemy::Enemy;

use crate::affect_response::AffectResponse;

///Representa la matriz que maneja el mapa
#[derive(Debug)]
pub struct Matrix {
    pub dimension: u32,
    data: Vec<Vec<MapObject>>,
}

impl Matrix {
    ///crea una nueva matriz en base al input dado en el formato de la consigna
    pub fn new(input: String, exec_err: &mut String) -> Option<Self> {
        let mut data: Vec<Vec<MapObject>> = Vec::new();
        let lines = input.split("\r\n").filter(|&x| !x.is_empty()); //sacar la linea vacia despues del ultimo salto de linea
        let dimension = lines.clone().count() as u32;

        for (i, line) in lines.enumerate() {
            let elements = line.split(' ');
            let row_dimension = elements.clone().count() as u32;
            if row_dimension != dimension {
                exec_err.push_str(
                    "La matriz debe ser cuadrada, (chequea no tener un salto de linea de más)",
                );
                return None;
            }
            let mut row: Vec<MapObject> = Vec::new();
            for (j, element) in elements.enumerate() {
                let map_object_result = map_object::MapObject::new(element, (j as u32, i as u32));
                match map_object_result {
                    Some(map_object) => {
                        row.push(map_object);
                    }
                    None => {
                        exec_err.push_str("hubo error al crear el objeto {element}");
                        return None;
                    }
                }
            }
            data.push(row);
        }
        Some(Self { dimension, data })
    }

    ///setea un nuevo valor en una posición especifica de la matriz
    pub fn set(&mut self, row: u32, col: u32, value: MapObject) {
        let row = u32_to_usize(row);
        let col = u32_to_usize(col);
        self.data[col][row] = value;
    }

    ///obtiene un valor de una posicion especifica de la matriz
    pub fn get(&self, row: u32, col: u32) -> MapObject {
        let row = u32_to_usize(row);
        let col = u32_to_usize(col);
        self.data[col][row].clone()
    }

    ///muestra la matriz en formato de string
    pub fn show(&self) -> String {
        let mut result = String::from("");
        for i in 0..(self.dimension) {
            for j in 0..(self.dimension) {
                let object = self.get(j, i); //van al revez para que se imprima como piden
                let string_object = object.show();
                result.push_str(&string_object);
                if j != self.dimension - 1 {
                    result.push(' ');
                }
            }
            result.push_str("\r\n");
        }
        result
    }

    ///decide como se afecta la posicion que se explota segun corresponda
    pub fn affect_position(
        &mut self,
        position_to_affect: (u32, u32),
        current_burst: &Burst,
    ) -> AffectResponse {
        let object_affected = self.get(position_to_affect.0, position_to_affect.1);
        match object_affected {
            MapObject::Nothing { id: _ } => AffectResponse::Continue,
            MapObject::Wall { id: _ } => AffectResponse::Stop,
            MapObject::Enemy(enemy) => {
                self.damage_enemy(enemy, position_to_affect, &current_burst.bomb)
            }
            MapObject::Rock { id: _ } => match current_burst.bomb.bomb_type {
                BombType::Shredding => AffectResponse::Continue,
                BombType::Normal => AffectResponse::Stop,
            },
            MapObject::Deviation { id: _, direction } => AffectResponse::Deviate { direction },
            MapObject::Bomb(bomb) => {
                /* Explode(bomba), la reemplazo en la matriz por un Nothing */
                self.set(
                    position_to_affect.0,
                    position_to_affect.1,
                    MapObject::Nothing {
                        id: String::from('_'),
                    },
                );
                AffectResponse::Explode { bomb }
            }
        }
    }

    ///daña al enemigo de una posición
    fn damage_enemy(
        &mut self,
        mut enemy: Enemy,
        position_to_affect: (u32, u32),
        bomb: &Bomb,
    ) -> AffectResponse {
        let is_killed = enemy.damage(bomb);
        match is_killed {
            Some(is_killed) => {
                if is_killed {
                    self.set(
                        position_to_affect.0,
                        position_to_affect.1,
                        MapObject::Nothing {
                            id: String::from('_'),
                        },
                    )
                } else {
                    self.set(
                        position_to_affect.0,
                        position_to_affect.1,
                        MapObject::Enemy(enemy),
                    );
                }
                AffectResponse::Continue
            }
            None => AffectResponse::AffectError {
                err: String::from("se intentó atacar a un muerto"),
            },
        }
    }

    // DEBUG
    // pub fn pretty_print(&self) {
    //     for i in 0..(self.dimension) {
    //         for j in 0..(self.dimension) {
    //             let object = self.get(j, i); //van al revez para que se imprima como piden
    //             object.pretty_print();
    //             if j != self.dimension - 1 {
    //                 print!(" ");
    //             }
    //         }
    //         print!("\r\n");
    //     }
    // }
}
