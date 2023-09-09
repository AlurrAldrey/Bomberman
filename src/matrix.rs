#[derive(Debug)]
pub struct Matrix {
    dimension: usize,
    data: Vec<Vec<String>>,
}

impl Matrix{

    pub fn new(input: String)-> Self {

        let mut data: Vec<Vec<String>> = Vec::new();
        let lines = input.split("\r\n");
        let dimension = lines.clone().count();

        for line in lines {

            let elements = line.split(" ");
            let row_dimension = elements.clone().count();
            if row_dimension != dimension { 
                //ver que hacer para resetear el programa
                println!("La matriz debe ser cuadrada, (chequea no tener un salto de linea de más)");
            }

            let mut row: Vec<String> = Vec::new();
            
            for element in elements{
                let unreferenced_element = String::from(element);
                row.push(unreferenced_element);
            }
            data.push(row);
        }
        Self {
            dimension, data
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: String) {
        if row > self.dimension || col > self.dimension { 
            print!("ERROR!: se intento acceder a un indice mayor a la dimansion de la matriz");
            return
        }

        self.data[row][col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> String {
        return self.data[row][col].clone();
    }
}