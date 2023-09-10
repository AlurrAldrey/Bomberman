pub mod map_object;
use self::map_object::MapObject;

#[derive(Debug)]
pub struct Matrix {
    pub dimension: usize,
    data: Vec<Vec<MapObject>>,
}

impl Matrix{

    pub fn new(input: String)-> Self {
        let mut data: Vec<Vec<MapObject>> = Vec::new();
        let lines = input.split("\r\n")
                                                        .filter(|&x| !x.is_empty()); //sacar la linea vacia despues del ultimo salto de linea
        let dimension = lines.clone().count();

        for line in lines {

            let elements = line.split(" ");
            let row_dimension = elements.clone().count();
            if row_dimension != dimension { 
                //ver que hacer para resetear el programa
                println!("La matriz debe ser cuadrada, (chequea no tener un salto de linea de más)");
            }

            let mut row: Vec<MapObject> = Vec::new();
            for element in elements{
                let map_object_result = map_object::MapObject::new(&element);
                match map_object_result {
                    Some(map_object) => {row.push(map_object);},
                    None => {println!("hubo error al crear el objeto")} //TODO: devolver None
                }
            }
            data.push(row);
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
}