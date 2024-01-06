mod c3d8;

#[macro_use]
mod maux;

use nalgebra as na;


fn main() {

    let e: f64 = 4500.0;
    let nu: f64 = 0.3;

    let nodes: Vec<na::Point3<f64>> = vec![
        nalgebra::Point3::new(0.0, 0.0, 0.0),
        nalgebra::Point3::new(1.0, 0.0, 0.0),
        nalgebra::Point3::new(1.0, 1.0, 0.0),
        nalgebra::Point3::new(0.0, 1.0, 0.0),
        nalgebra::Point3::new(0.0, 0.0, 1.0),
        nalgebra::Point3::new(1.0, 0.0, 1.0),
        nalgebra::Point3::new(1.0, 1.0, 1.0),
        nalgebra::Point3::new(0.0, 1.0, 1.0),
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

    mprint!(ke);
}
