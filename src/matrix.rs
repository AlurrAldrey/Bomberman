pub mod map_object;
use self::map_object::MapObject;

use crate::burst::Burst; 

#[derive(Debug)]
pub struct Matrix {
    pub dimension: usize,
    data: Vec<Vec<MapObject>>,
}

pub enum AffectResponse{
    Deviate{ direction: char }, 
    Explode { bomb: MapObject}, 
    Stop, 
    Continue
}

impl Matrix{

    pub fn new(input: String)-> Self {
        let mut data: Vec<Vec<MapObject>> = Vec::new();
        let lines = input.split("\r\n")
                                                        .filter(|&x| !x.is_empty()); //sacar la linea vacia despues del ultimo salto de linea
        let dimension = lines.clone().count();

        let mut i = 0;
        for line in lines {

            let elements = line.split(" ");
            let row_dimension = elements.clone().count();
            if row_dimension != dimension { 
                //ver que hacer para resetear el programa
                println!("La matriz debe ser cuadrada, (chequea no tener un salto de linea de más)");
            }

            let mut row: Vec<MapObject> = Vec::new();
            let mut j = 0;
            for element in elements{
                let map_object_result = map_object::MapObject::new(&element, (i,j));
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

    pub fn set(&mut self, row: usize, col: usize, value: MapObject) {
        if row > self.dimension || col > self.dimension { 
            print!("ERROR!: se intento acceder a un indice mayor a la dimansion de la matriz");
            return
        }

        self.data[row][col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> &MapObject {
        return &self.data[row][col];
    }

    pub fn pretty_print(&self) {
        for i in 0..(self.dimension) {
            for j in 0..(self.dimension) {
                let object = self.get(i,j);
                object.pretty_print();
                if j != self.dimension -1 {print!(" ");}
            }
            print!("\r\n");
        }
    }

    pub fn affect_position(&self, position_to_affect: (usize,usize), current_burst: &Burst) -> AffectResponse {
        let object_affected = self.get(position_to_affect.0.clone(), position_to_affect.1.clone());
        match object_affected {
            MapObject::Nothing { id } => { return AffectResponse::Continue },
            MapObject::Wall{id} =>{ return AffectResponse::Stop },
            MapObject::Enemy { id, health } => { /*restarle vida al enemigo y matarlo si es el caso
                                                                luego devolver un Continue*/
                object_affected.pretty_print()},
            MapObject::Rock { id } => {/* Continue si es Shredding, Stop si es Normal */},
            MapObject::Deviation { id, direction } => {/* Deviate con la direccion correspondiente*/},
            MapObject::Bomb { id, range, bomb_type, position } => {/* Explode(bomba), la reemplazo en la matriz por un Nothing */}
        }
    }
}