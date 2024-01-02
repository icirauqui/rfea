mod c3d8;

#[macro_use]
mod maux;


fn main() {

    let e: f64 = 4500.0;
    let nu: f64 = 0.3;

    let hexa = c3d8::C3D8::new(vec![
        nalgebra::Point3::new(0.0, 0.0, 0.0),
        nalgebra::Point3::new(1.0, 0.0, 0.0),
        nalgebra::Point3::new(1.0, 1.0, 0.0),
        nalgebra::Point3::new(0.0, 1.0, 0.0),
        nalgebra::Point3::new(0.0, 0.0, 1.0),
        nalgebra::Point3::new(1.0, 0.0, 1.0),
        nalgebra::Point3::new(1.0, 1.0, 1.0),
        nalgebra::Point3::new(0.0, 1.0, 1.0),
    ], e, nu);

    let d = hexa.get_d();
    mprint!(d);

    println!();

    let gauss_points = [-0.577350269189626, 0.577350269189626];

    for xi in gauss_points {
        println!("xi: {}", xi);
    }
}
