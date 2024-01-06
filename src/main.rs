
use std::io::prelude::*;

mod c3d8;
mod data;

#[macro_use]
mod maux;

use nalgebra as na;




fn main() {

    let e: f64 = 3500.0;
    let nu: f64 = 0.495;

    let nodes: Vec<na::Point3<f64>> = vec![
        nalgebra::Point3::new(0.0, 0.0, 0.0),
        nalgebra::Point3::new(1.0, 0.0, 0.0),
        nalgebra::Point3::new(0.0, 1.0, 0.0),
        nalgebra::Point3::new(1.0, 1.0, 0.0),
        nalgebra::Point3::new(0.0, 0.0, 1.0),
        nalgebra::Point3::new(1.0, 0.0, 1.0),
        nalgebra::Point3::new(0.0, 1.0, 1.0),
        nalgebra::Point3::new(1.0, 1.0, 1.0),
    ];

    //let d = hexa.get_d();
    //mprint!(d);
    //println!();
    //let gauss_points = [-0.577350269189626, 0.577350269189626];
    //for xi in gauss_points {
    //    println!("xi: {}", xi);
    //}
    
    let hexa = c3d8::C3D8::new(e, nu);

    let ke = hexa.ke(nodes);

    println!("ke");
    println!("{}", ke);

    let res = data::save_mat_to_file(&ke, "./data/ke.csv");
    println!("{:?}", res);

    // Save stiffness matrix to file
    //let mut file = std::fs::File::create("../data/ke.csv").unwrap();
    //for i in 0..ke.shape().0 {
    //    for j in 0..ke.shape().1 {
    //        if j == ke.shape().1 - 1 {
    //            file.write_fmt(format_args!("{}", ke[(i, j)])).unwrap();
    //            continue;
    //        }
    //        file.write_fmt(format_args!("{},", ke[(i, j)])).unwrap();
    //    }
    //    file.write_fmt(format_args!("\n")).unwrap();
    //}
    
}
