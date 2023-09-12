use std::fs;

mod matrix;
use matrix::map_object::MapObject;

mod burst;
use burst::Burst;

mod helpers;
use helpers::increment_burst_position;

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

    let matrix = matrix::Matrix::new(contents);
    let map_object_exploded: &MapObject = matrix.get(first_explosion.0, first_explosion.1);
    let first_bomb: &MapObject;
    let first_bomb_range;
    match map_object_exploded {
        MapObject::Bomb { id, range , bomb_type, position} => {
            first_bomb_range = range;
            first_bomb = &MapObject::Bomb { 
                id: id.clone(), 
                range: range.clone(), 
                bomb_type: bomb_type.clone(), 
                position: position.clone() };
         },
        _ => { println!("no se explotó una bomba"); return ()}
    };
    
    //vector con las rafagas que se van a efectuar
    let mut burst_queue: Vec<Burst> = Vec::new();

    //cargo las 4 rafagas que generará la primer bomba
    //TODO: agregar el identificador de la bomba a la que pertenece la rafaga
    burst_queue.push(Burst::new('U', first_explosion, first_bomb_range.clone(), first_bomb.clone()));
    burst_queue.push(Burst::new('R', first_explosion, first_bomb_range.clone(), first_bomb.clone()));
    burst_queue.push(Burst::new('D', first_explosion, first_bomb_range.clone(), first_bomb.clone()));
    burst_queue.push(Burst::new('L', first_explosion, first_bomb_range.clone(), first_bomb.clone()));

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

            //incremento la posición en el eje que corresponda
            let position_to_affect = increment_burst_position(current_burst.direction,
                                                                                     current_burst.starting_position.clone(), 
                                                                                     i_us, 
                                                                                     &matrix.dimension);
            match position_to_affect {
                Some(position_to_affect) => {
                    matrix.affect_position(position_to_affect, &current_burst);
                    //en caso de que haya que desviar la rafaga se desvia agregando una nueva al burst_queue
                    //en caso de que haya que explotar una bomba, se agregan nuevas rafagas a la queue
                    //en caso de que haya que frenar la rafaga porque encuentra una pared/roca, se llama a break
                    //en caso de que haya que continuar, se sigue con la iteración
                    //Deviate(direction), Explode(bomb), Stop(), Continue();
                },
                None => { break; }
            }
            // matrix.affect(current_burst, burst_position);
        }
    }
}
