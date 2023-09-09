// use std::env;
use std::fs;

#[path = "matrix/map_object.rs"] mod map_object;
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
    println!("{contents}");

    let matrix = matrix::Matrix::new(contents);
    println!("{:?}", matrix);

    matrix.pretty_print();

    let object: String = String::from("DU");
    let map_object: Option<map_object::MapObject> = map_object::MapObject::new(&object);
    let map_object: map_object::MapObject = match map_object {
        Some(map_object) => map_object,
        None => {
            println!("No se pudo generar la clase");
            // main();
            return ();
        },
    };

    println!("{:?}", map_object);
}
