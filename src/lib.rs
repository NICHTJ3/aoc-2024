pub mod template;

pub fn out_of_bounds<T>(y: i32, x: i32, matrix: &[Vec<T>]) -> bool {
    y < 0 || x < 0 || y >= matrix.len() as i32 || x >= matrix[0].len() as i32
}

// Use this file to add helper functions and additional modules.
