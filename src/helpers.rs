pub fn increment_burst_position(burst_direction: char, burst_position: (u32,u32), i: u32, matrix_dimension: &u32) -> Option<(u32,u32)>{
    
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

pub fn u32_to_usize(n:u32) -> usize {
    //paso a i de u32 a usize
    let mut n_us = 0;
    match usize::try_from(n)  {
        Ok(result) => { n_us = result },
        Err(_) => {}
    }
    return n_us;
}