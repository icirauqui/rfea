
use std::{fs::File, io::Write};
use nalgebra as na;

pub fn save_mat_to_file(mat: &na::SMatrix<f64, 24, 24>, filename: &str) -> std::io::Result<()> {
    let mut file = File::create(filename)?;
    for i in 0..mat.shape().0 {
        for j in 0..mat.shape().1 {
            //println!("{} {}: {}", i, j, mat[(i, j)]);
            let text: String = format!("{}\t", mat[(i, j)]);
            file.write_all(text.as_bytes())?;
        }
        file.write_all(b"\n")?;
    }
    Ok(())
}