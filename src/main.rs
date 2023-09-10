#[path = "matrix/map_object.rs"] mod map_object;
mod matrix;
use matrix::map_object::MapObject;

mod burst;
use burst::Burst;

mod helpers;
use helpers::increment_burst_position;

use std::fs;


fn main() {

    let first_explosion = (0,0);
    //tomarlo como parametro del programa
    let file_path: String = String::from("C:/Users/Usuario/Desktop/taller/bomberman/src/input_tests/test1.txt");
    let contents: Result<String, std::io::Error> = fs::read_to_string(file_path);
    let contents = match contents {
        Ok(contents) => contents,
        Err(error) => {
            println!("Hubo un error al leer el archivo, intentelo denuevo: {:?}", error);
            // main();
            return ();
        },
    };

    let mut matrix = matrix::Matrix::new(contents);
    let map_object_exploded: &matrix::map_object::MapObject = matrix.get(first_explosion.0, first_explosion.1);
    let first_bomb_range;
    match map_object_exploded {
        MapObject::Bomb { id, range , bomb_type} => { first_bomb_range = range },
        _ => {return ()}
    };
    
    //vector con las rafagas que se van a efectuar
    let mut burst_queue: Vec<Burst> = Vec::new();

    //cargo las 4 rafagas que generará la primer bomba
    burst_queue.push(Burst::new('U', first_explosion, first_bomb_range.clone()));
    burst_queue.push(Burst::new('R', first_explosion, first_bomb_range.clone()));
    burst_queue.push(Burst::new('D', first_explosion, first_bomb_range.clone()));
    burst_queue.push(Burst::new('L', first_explosion, first_bomb_range.clone()));

    while burst_queue.len() > 0 {
        println!("{:?}", burst_queue[0]);
        let current_burst = burst_queue.remove(0);

        //recorro casillero por casillero los lugares afectados por la rafaga
        for i in 0..(current_burst.range + 1)  {

            //paso a i de u32 a usize
            let mut i_us = 0;
            match usize::try_from(i)  {
                Ok(result) => { i_us = result },
                Err(_) => {}
            }
            let position_to_affect = increment_burst_position(current_burst.direction,
                                                                                     current_burst.starting_position, 
                                                                                     i_us, 
                                                                                     &matrix.dimension);
            match position_to_affect {
                Some(position_to_affect) => {println!("{:?}", position_to_affect);},
                None => { break; }
            }
            // matrix.affect(current_burst, burst_position);
        }
    }
}
