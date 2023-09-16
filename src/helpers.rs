use crate::burst::Burst;
use crate::Bomb;
use std::env;
use std::fs;
use std::io::Write;

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
    fs::create_dir_all(output_dir);

    //Creo el archivo de output en el directorio
    let output_file_path = format!("{}/output.txt", output_dir);
    let mut output_file =
        fs::File::create(&output_file_path).expect("Failed to create output file");

    //leo el input
    let read = fs::read_to_string(input_file);
    let file_contents: String;
    match read {
        Ok(content) => file_contents = content,
        Err(err) => {
            writeln!(output_file, "Leer el archivo de input");
            return None;
        }
    }

    let parse_x_y = parse_x_y(args, &output_file);
    match parse_x_y {
        Some((x, y)) => {
            return Some((file_contents, output_file, x, y));
        }
        None => return None,
    }
}

fn parse_x_y(args: Vec<String>, mut output_file: &fs::File) -> Option<(u32, u32)> {
    let parse_x = args[3].parse::<u32>();
    let x;
    match parse_x {
        Ok(res) => x = res,
        Err(err) => {
            writeln!(output_file, "No se pudo interpretar 'x'");
            return None;
        }
    }
    let parse_y = args[4].parse::<u32>();
    let y;
    match parse_y {
        Ok(res) => y = res,
        Err(err) => {
            writeln!(output_file, "No se pudo interpretar 'y'");
            return None;
        }
    }
    return Some((x, y));
}

pub fn increment_burst_position(
    burst_direction: char,
    burst_position: (u32, u32),
    i: u32,
    matrix_dimension: &u32,
) -> Option<(u32, u32)> {
    //chequeo que no se intente hacer una explosion fuera de las dimensiones de la matriz
    let mut new_position = burst_position;
    match burst_direction {
        'D' => {
            if burst_position.1 + i < matrix_dimension.clone() {
                new_position = (burst_position.0, burst_position.1 + i)
            } else {
                return None;
            }
        }
        'R' => {
            if burst_position.0 + i < matrix_dimension.clone() {
                new_position = (burst_position.0 + i, burst_position.1)
            } else {
                return None;
            }
        }
        'U' => {
            if let Some(result) = burst_position.1.checked_sub(i) {
                new_position = (burst_position.0, result)
            } else {
                return None;
            };
        }
        'L' => {
            if let Some(result) = burst_position.0.checked_sub(i) {
                new_position = (result, burst_position.1)
            } else {
                return None;
            };
        }
        _ => return None,
    }

    return Some(new_position);
}

pub fn u32_to_usize(n: u32) -> usize {
    //paso a i de u32 a usize
    let mut n_us = 0;
    match usize::try_from(n) {
        Ok(result) => n_us = result,
        Err(_) => {}
    }
    return n_us;
}

//Carga 4 rafagas correspondientes a una bomba
pub fn load_bomb_bursts(burst_queue: &mut Vec<Burst>, bomb: Bomb) {
    burst_queue.push(Burst::new(
        'U',
        bomb.position,
        bomb.range.clone(),
        bomb.clone(),
    ));
    burst_queue.push(Burst::new(
        'R',
        bomb.position,
        bomb.range.clone(),
        bomb.clone(),
    ));
    burst_queue.push(Burst::new(
        'D',
        bomb.position,
        bomb.range.clone(),
        bomb.clone(),
    ));
    burst_queue.push(Burst::new('L', bomb.position, bomb.range.clone(), bomb));
}

pub fn get_u32_from_char(number_char: Option<char>, string_rep: &str) -> u32 {
    let mut number_unsigned: u32 = 0;
    match number_char {
        Some(health) => {
            let health_todigit = health.to_digit(10);
            match health_todigit {
                Some(digit) => number_unsigned = digit,
                None => {
                    println!("El valor de {string_rep} no es un numero")
                }
            }
        }
        None => {
            println!("No se encontró el valor de {string_rep}")
        }
    };
    number_unsigned
}
