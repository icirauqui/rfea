use nalgebra as na;


pub type Matrix3x8<T> = na::Matrix<T, na::Const<3>, na::Const<8>, na::ArrayStorage<T, 3, 8>>;



pub struct C3D8 {
    _nodes: Vec<na::Point3<f64>>,
    _d: na::Matrix6<f64>,
}

impl C3D8 {
    pub fn new(_nodes: Vec<na::Point3<f64>>, e: f64, nu: f64) -> Self {

        let lambda = (e*nu) / ((1.0 + nu)*(1.0 - 2.0*nu));
        let mu = e / (2.0*(1.0 + nu));

        let _d: na::Matrix6<f64> = na::Matrix6::new(
            lambda + 2.0*mu, lambda, lambda, 0.0, 0.0, 0.0,
            lambda, lambda + 2.0*mu, lambda, 0.0, 0.0, 0.0,
            lambda, lambda, lambda + 2.0*mu, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, mu, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0, mu, 0.0,
            0.0, 0.0, 0.0, 0.0, 0.0, mu,
        );

        Self { _nodes,  _d }
    }

    pub fn ke(&self) {
        let gauss_points = [-0.577350269189626, 0.577350269189626];

        for xi in gauss_points {
            for eta in gauss_points {
                for zeta in gauss_points {
                    let dShape: na::Matrix3x6<f64>;


                    let dShape: na::Matrix<f64, na::Const<3>, na::Const<8>, na::ArrayStorage<f64, 3, 8>> = na::Matrix::<f64, na::Const<3>, na::Const<8>, na::ArrayStorage<f64, 3, 8>>::new(
                        -0.125*(1.0 - eta)*(1.0 - zeta), 
                         0.125*(1.0 - eta)*(1.0 - zeta), 
                         0.125*(1.0 + eta)*(1.0 - zeta), 
                        -0.125*(1.0 + eta)*(1.0 - zeta), 
                        -0.125*(1.0 - eta)*(1.0 + zeta), 
                         0.125*(1.0 - eta)*(1.0 + zeta), 
                         0.125*(1.0 + eta)*(1.0 + zeta), 
                        -0.125*(1.0 + eta)*(1.0 + zeta),

                        -0.125*(1.0 - xi)*(1.0 - zeta), 
                        -0.125*(1.0 + xi)*(1.0 - zeta), 
                         0.125*(1.0 + xi)*(1.0 - zeta), 
                         0.125*(1.0 - xi)*(1.0 - zeta), 
                        -0.125*(1.0 - xi)*(1.0 + zeta), 
                        -0.125*(1.0 + xi)*(1.0 + zeta), 
                         0.125*(1.0 + xi)*(1.0 + zeta), 
                         0.125*(1.0 - xi)*(1.0 + zeta),

                        -0.125*(1.0 - xi)*(1.0 - eta), 
                        -0.125*(1.0 + xi)*(1.0 - eta), 
                        -0.125*(1.0 + xi)*(1.0 + eta), 
                        -0.125*(1.0 - xi)*(1.0 + eta), 
                         0.125*(1.0 - xi)*(1.0 - eta), 
                         0.125*(1.0 + xi)*(1.0 - eta), 
                         0.125*(1.0 + xi)*(1.0 + eta), 
                         0.125*(1.0 - xi)*(1.0 + eta),
                    );
                }
            }
        }
    }


    pub fn get_d(&self) -> &na::Matrix6<f64> {        
        &self._d
    }
}