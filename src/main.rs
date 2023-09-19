mod tests;

use std::fs;
use std::io::Write;

mod enemy;
mod map_object;
mod matrix;

mod affect_response;
use affect_response::AffectResponse;

mod bomb;
use bomb::Bomb;

mod burst;
use burst::Burst;

mod helpers;
use helpers::get_args_from_call;
use helpers::increment_burst_position;
use helpers::load_bomb_bursts;
use helpers::initialize_burst_queue;
use helpers::write_file;

use matrix::Matrix;


fn main() {
    let args_result = get_args_from_call();
    let contents;
    let output_file: fs::File;
    let first_explosion;
    let mut exec_err: String = String::from("");
    match args_result {
        Some((file_content, output, x, y)) => {
            contents = file_content;
            output_file = output;
            first_explosion = (x, y);
        }
        None => return,
    }
    let build_matrix = matrix::Matrix::new(contents, &mut exec_err);
    let mut matrix: matrix::Matrix;
    match build_matrix {
        Some(matrix_result) => { matrix = matrix_result },
        None => { 
            write_file(output_file, exec_err);
            return;
        } 
    }

    let get_burst_queue = initialize_burst_queue(first_explosion, &mut matrix, &mut exec_err); //vector con las rafagas que se van a efectuar
    match get_burst_queue {
        Some(burst_queue) => { iterate_bursts(burst_queue, matrix, output_file); },
        None => {
            write_file(output_file, exec_err);
            return
        }
    }
    println!("Fin de la ejecución");
}

//No tengo forma de acortar este matodo dado que cargo fmt me agrega muchas lineas
fn iterate_bursts(mut burst_queue: Vec<Burst>, mut matrix: matrix::Matrix, mut output_file: fs::File) {
    while !burst_queue.is_empty() {
        let current_burst = burst_queue.remove(0);
        //recorro casillero por casillero los lugares afectados por la rafaga
        for i in 1..(current_burst.range + 1) {
            let position_to_affect = increment_burst_position(//incremento la posición en el eje que corresponda
                current_burst.direction,
                current_burst.starting_position,
                i,
                &matrix.dimension,
            );
            match position_to_affect {
                Some(position_to_affect) => {
                    let response = matrix.affect_position(position_to_affect, &current_burst);
                    match response {
                        AffectResponse::Explode { bomb } => {
                            load_bomb_bursts(&mut burst_queue, bomb);
                        }
                        AffectResponse::Continue => continue,
                        AffectResponse::Stop => break,
                        AffectResponse::Deviate { direction } => {
                            burst_queue.push(Burst::new(
                                direction,
                                position_to_affect,
                                current_burst.range + 1 - i,
                                current_burst.bomb.clone(),
                            ));
                        }
                        AffectResponse::AffectError { err } => {writeln!(output_file, "{err}").unwrap_or(println!("{err}"));} //manejar error
                    }
                }
                None => {
                    break;//la posición calculada estaba fuera de la matriz
                }
            }
        }
    }
    write_file(output_file, matrix.to_string());
}