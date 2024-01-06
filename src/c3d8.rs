use nalgebra as na;


pub type Matrix3x8<T> = na::Matrix<T, na::Const<3>, na::Const<8>, na::ArrayStorage<T, 3, 8>>;
pub type Matrix8x3<T> = na::Matrix<T, na::Const<8>, na::Const<3>, na::ArrayStorage<T, 8, 3>>;
pub type Matrix24x24<T> = na::Matrix<T, na::Const<24>, na::Const<24>, na::ArrayStorage<T, 24, 24>>;


pub struct C3D8 {
    _d: na::Matrix6<f64>,
}

impl C3D8 {
    pub fn new(e: f64, nu: f64) -> Self {

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



        Self { _d }
    }

    fn mat_d(&self, xi: f64, eta: f64, zeta: f64) -> na::Matrix<f64, na::Const<3>, na::Const<8>, na::ArrayStorage<f64, 3, 8>> {
        let mat_d: na::Matrix<f64, na::Const<3>, na::Const<8>, na::ArrayStorage<f64, 3, 8>> = na::Matrix::<f64, na::Const<3>, na::Const<8>, na::ArrayStorage<f64, 3, 8>>::new(
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
        //return dShape;
        mat_d
    }

    fn mat_b(&self, mat_d_dx: Matrix3x8<f64>) -> na::Matrix<f64, na::Const<6>, na::Const<24>, na::ArrayStorage<f64, 6, 24>> {
        let mat_b: na::Matrix<f64, na::Const<6>, na::Const<24>, na::ArrayStorage<f64, 6, 24>> = na::Matrix::<f64, na::Const<6>, na::Const<24>, na::ArrayStorage<f64, 6, 24>>::new(
            mat_d_dx[(0, 0)],              0.0,              0.0, mat_d_dx[(0, 1)], 0.0, 0.0, mat_d_dx[(0, 2)], 0.0, 0.0, mat_d_dx[(0, 3)], 0.0, 0.0, mat_d_dx[(0, 4)], 0.0, 0.0, mat_d_dx[(0, 5)], 0.0, 0.0, mat_d_dx[(0, 6)], 0.0, 0.0, mat_d_dx[(0, 7)], 0.0, 0.0,
                         0.0, mat_d_dx[(1, 0)],              0.0, 0.0, mat_d_dx[(1, 1)], 0.0, 0.0, mat_d_dx[(1, 2)], 0.0, 0.0, mat_d_dx[(1, 3)], 0.0, 0.0, mat_d_dx[(1, 4)], 0.0, 0.0, mat_d_dx[(1, 5)], 0.0, 0.0, mat_d_dx[(1, 6)], 0.0, 0.0, mat_d_dx[(1, 7)], 0.0,
                         0.0,              0.0, mat_d_dx[(2, 0)], 0.0, 0.0, mat_d_dx[(2, 1)], 0.0, 0.0, mat_d_dx[(2, 2)], 0.0, 0.0, mat_d_dx[(2, 3)], 0.0, 0.0, mat_d_dx[(2, 4)], 0.0, 0.0, mat_d_dx[(2, 5)], 0.0, 0.0, mat_d_dx
        );
        mat_b
    }

    pub fn ke(&self, _nodes: Vec<na::Point3<f64>>,) -> Matrix24x24<f64> {

        let coordinates: Matrix8x3<f64> = na::Matrix::<f64, na::Const<8>, na::Const<3>, na::ArrayStorage<f64, 8, 3>>::new(
            _nodes[0].x, _nodes[0].y, _nodes[0].z,
            _nodes[1].x, _nodes[1].y, _nodes[1].z,
            _nodes[2].x, _nodes[2].y, _nodes[2].z,
            _nodes[3].x, _nodes[3].y, _nodes[3].z,
            _nodes[4].x, _nodes[4].y, _nodes[4].z,
            _nodes[5].x, _nodes[5].y, _nodes[5].z,
            _nodes[6].x, _nodes[6].y, _nodes[6].z,
            _nodes[7].x, _nodes[7].y, _nodes[7].z,
        );


        let gauss_points = [-0.577350269189626, 0.577350269189626];
        let mut _ke = Matrix24x24::<f64>::zeros();

        for xi in gauss_points {
            for eta in gauss_points {
                for zeta in gauss_points {
                    let mat_d = self.mat_d(xi, eta, zeta);
                    let jacobian = mat_d * coordinates;
                    let jacobian_inverse = jacobian.try_inverse().unwrap();
                    let mat_d_dx = jacobian_inverse * mat_d;

                    let mat_b = self.mat_b(mat_d_dx);

                    let _ke_i = mat_b.transpose() * self._d * mat_b;
                    _ke += _ke_i * jacobian.determinant();
                }
            }
        }

        _ke
    }


    pub fn get_d(&self) -> &na::Matrix6<f64> {        
        &self._d
    }
}