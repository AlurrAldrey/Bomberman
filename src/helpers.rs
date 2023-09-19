use crate::burst::Burst;
use crate::AffectResponse;
use crate::Bomb;
use crate::Matrix;
use std::env;
use std::fs;
use std::io::Write;

pub fn write_file(mut output_file: fs::File, exec_err: String) {
    let write = writeln!(output_file, "{exec_err}");
    match write {
        Ok(()) => {}
        Err(err) => println!("{err}"),
    }
}

pub fn get_args_from_call() -> Option<(String, fs::File, u32, u32)> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() != 5 {
        println!("El formato debe ser: cargo run -- <input_file> <output_directory> <x> <y>");
        return None;
    }

    let input_file = &args[1];
    let output_dir = &args[2];

    //Creo el directorio de output si no existe
    fs::create_dir_all(output_dir).unwrap_or(println!("creacion del directorio de output"));

    //Creo el archivo de output en el directorio
    let output_file_path = format!("{}/output.txt", output_dir);
    let create_file = fs::File::create(output_file_path);
    let output_file: fs::File = match create_file {
        Ok(file) => file,
        Err(err) => {
            println!("{err}");
            return None;
        }
    };

    //leo el input
    let read = fs::read_to_string(input_file);

    let file_contents: String = match read {
        Ok(content) => content,
        Err(err) => {
            let error_string = format!("{err}");
            write_file(output_file, error_string);
            return None;
        }
    };

    let parse_x_y = parse_x_y(args, &output_file);
    parse_x_y.map(|(x, y)| (file_contents, output_file, x, y))
}

fn parse_x_y(args: Vec<String>, mut output_file: &fs::File) -> Option<(u32, u32)> {
    let parse_x = args[3].parse::<u32>();

    let x = match parse_x {
        Ok(res) => res,
        Err(err) => {
            writeln!(output_file, "No se pudo interpretar 'x' {err}").unwrap_or(println!("{err}"));
            return None;
        }
    };
    let parse_y = args[4].parse::<u32>();

    let y = match parse_y {
        Ok(res) => res,
        Err(err) => {
            writeln!(output_file, "No se pudo interpretar 'y' {err}").unwrap_or(println!("{err}"));
            return None;
        }
    };
    Some((x, y))
}

pub fn increment_burst_position(
    burst_direction: char,
    burst_position: (u32, u32),
    i: u32,
    matrix_dimension: &u32,
) -> Option<(u32, u32)> {
    //chequeo que no se intente hacer una explosion fuera de las dimensiones de la matriz
    // let mut new_positionk = burst_position;
    match burst_direction {
        'D' => {
            if burst_position.1 + i < *matrix_dimension {
                Some((burst_position.0, burst_position.1 + i))
            } else {
                None
            }
        }
        'R' => {
            if burst_position.0 + i < *matrix_dimension {
                Some((burst_position.0 + i, burst_position.1))
            } else {
                None
            }
        }
        'U' => burst_position
            .1
            .checked_sub(i)
            .map(|result| (burst_position.0, result)),
        'L' => burst_position
            .0
            .checked_sub(i)
            .map(|result| (result, burst_position.1)),
        _ => None,
    }
}

pub fn u32_to_usize(n: u32) -> usize {
    //paso a i de u32 a usize
    let mut n_us = 0;
    if let Ok(result) = usize::try_from(n) {
        n_us = result
    }
    n_us
}

//Carga 4 rafagas correspondientes a una bomba
pub fn load_bomb_bursts(burst_queue: &mut Vec<Burst>, bomb: Bomb) {
    burst_queue.push(Burst::new('U', bomb.position, bomb.range, bomb.clone()));
    burst_queue.push(Burst::new('R', bomb.position, bomb.range, bomb.clone()));
    burst_queue.push(Burst::new('D', bomb.position, bomb.range, bomb.clone()));
    burst_queue.push(Burst::new('L', bomb.position, bomb.range, bomb));
}

pub fn get_u32_from_char(number_char: Option<char>) -> Option<u32> {
    match number_char {
        Some(health) => health.to_digit(10),
        None => None,
    }
}

pub fn initialize_burst_queue(
    first_explosion: (u32, u32),
    matrix: &mut Matrix,
    exec_err: &mut String,
) -> Option<Vec<Burst>> {
    let mut burst_queue: Vec<Burst> = Vec::new(); //vector con las rafagas que se van a efectuar
    let first_spark = Bomb::new(first_explosion); //representa la 'chispa' que explota la primera bomba
    let first_response = matrix.affect_position(
        first_explosion,
        &Burst::new('U', first_explosion, 0, first_spark),
    );
    match first_response {
        AffectResponse::Explode { bomb } => {
            load_bomb_bursts(&mut burst_queue, bomb);
            Some(burst_queue)
        }
        _ => {
            exec_err.push_str("no se explotó una bomba");
            None
        }
    }
}
