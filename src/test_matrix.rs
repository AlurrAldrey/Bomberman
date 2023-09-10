fn run_matrix_test() {
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

    let mut matrix = matrix::Matrix::new(contents);
    println!("{:?}", matrix);

    let pos0 = matrix.get(0, 0);
    println!("{:?}", pos0);

    matrix.set(0,0, matrix::map_object::MapObject::Enemy { id: String::from("El duende loco"), health: 1 });

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