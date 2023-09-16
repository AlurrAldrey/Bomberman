use std::fs;

mod enemy;
mod map_object;
mod matrix;

mod affect_response;
use affect_response::AffectResponse;

mod bomb;
use bomb::Bomb;
use bomb::BombType;

mod burst;
use burst::Burst;

mod helpers;
use helpers::get_args_from_call;
use helpers::increment_burst_position;
use helpers::load_bomb_bursts;

fn main() {
    let args_result = get_args_from_call();
    let contents;
    let output_file: fs::File;
    let first_explosion;
    match args_result {
        Some((file_content, output, x, y)) => {
            contents = file_content;
            output_file = output;
            first_explosion = (x, y);
        }
        None => return,
    }
    println!("{contents}");

    let mut matrix = matrix::Matrix::new(contents);
    matrix.pretty_print(); //DEBUG

    //vector con las rafagas que se van a efectuar
    let mut burst_queue: Vec<Burst> = Vec::new();
    let first_spark = Bomb {
        id: String::from('*'),
        range: 0,
        bomb_type: BombType::Normal,
        position: first_explosion,
    }; //representa la 'chispa' que explota la primera bomba
    let first_response = matrix.affect_position(
        first_explosion,
        &Burst::new('U', first_explosion, 0, first_spark),
    );
    match first_response {
        AffectResponse::Explode { bomb } => {
            load_bomb_bursts(&mut burst_queue, bomb);
        }
        _ => {
            println!("no se explotó una bomba");
            return ();
        }
    }

    //partir en 2 aca
    while burst_queue.len() > 0 {
        let current_burst = burst_queue.remove(0);

        //recorro casillero por casillero los lugares afectados por la rafaga
        for i in 1..(current_burst.range + 1) {
            //incremento la posición en el eje que corresponda
            let position_to_affect = increment_burst_position(
                current_burst.direction,
                current_burst.starting_position.clone(),
                i,
                &matrix.dimension,
            );
            match position_to_affect {
                Some(position_to_affect) => {
                    println!("{position_to_affect:?}");
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
                        _ => todo!(), //manejar error
                    }
                }
                None => {
                    break;
                }
            }
        }
        matrix.pretty_print(); //DEBUG
    }
}
