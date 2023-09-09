// use std::env;
use std::fs;

mod map_object;
mod matrix;

fn main() {

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
    println!("{:?}", matrix);

    matrix.set(0,0, String::from("HOLA"));
    println!("{:?}", matrix);

    let position_zero = matrix.get(0,0);
    println!("{:?}", position_zero);

    let wall: String = String::from("W");
    let wall_object = map_object::MapObject::new(&wall);
    let wall_object = match wall_object {
        Some(map_object) => map_object,
        None => {
            println!("No se pudo generar la clase");
            // main();
            return ();
        },
    };

    println!("{:?}", wall_object);
}
