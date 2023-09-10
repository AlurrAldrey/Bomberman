pub fn increment_burst_position(burst_direction: char, burst_position: (usize,usize), i: usize, matrix_dimension: &usize) -> Option<(usize,usize)>{
    
    //chequeo que no se intente hacer una explosion fuera de las dimensiones de la matriz
    let mut new_position = burst_position;
    match burst_direction {
        'U' => { 
            if burst_position.1 + i <= matrix_dimension.clone()  { new_position = (burst_position.0, burst_position.1 + i)}
            else { return None }
         },
        'R' => { 
            if burst_position.0 + i <= matrix_dimension.clone()  { new_position = (burst_position.0 + i, burst_position.1)}
            else { return None }
         },
        'D' => { 
            if let Some(result) = burst_position.1.checked_sub(i) { new_position = (burst_position.0, result)}
            else { return None };
         },
        'L' => { 
            if let Some(result) = burst_position.0.checked_sub(i) { new_position = (burst_position.0, result)}
            else { return None };
        },
        _=> { return None } 
    }
    
    return Some(new_position);
}