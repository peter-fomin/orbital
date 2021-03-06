use std::f64::consts::PI;

use druid::{Data, Lens};

use crate::Orbit;
use crate::Vector3D;

#[derive(Debug, Clone, Data, PartialEq)]
pub enum LambertSolverStatus {
    Initialized,
    Converged,
    NotConverged,
    MultiRevolution,
    CollinearVectors,
    InvalidInput,
    InternalError,
}

impl Default for LambertSolverStatus {
    fn default() -> Self {
        Self::Initialized
    }
}

#[derive(Default, Data, Clone, Lens)]
pub struct LambertSolver {
    // first radius-vector
    r1_v: Vector3D,
    i_r1: Vector3D,
    r1: f64,

    // second radius-vector
    r2_v: Vector3D,
    i_r2: Vector3D,
    r2: f64,

    // time of flight
    t: f64,

    // gravitational body constant
    mu: f64,

    // chorde
    c_v: Vector3D,
    c: f64,

    // half of perimeter
    s: f64,

    lambda: f64,
    i_h: Vector3D,

    i_t1: Vector3D,
    i_t2: Vector3D,

    // non dimensional time-of-flight
    t_nd: f64,

    // fitting parameter
    x: f64,
    y: f64,

    // Output parameters
    pub status: LambertSolverStatus,
    v1: Vector3D,
    v2: Vector3D,
}

impl LambertSolver {
    pub fn new(r1: Vector3D, r2: Vector3D, t: f64, mu: f64) -> Result<Self, LambertSolverStatus> {
        let mut new = Self {
            r1_v: r1,
            r2_v: r2,
            t: t,
            mu: mu,
            ..Default::default()
        };
        match new.calculate_params() {
            Ok(_) => Ok(new),
            Err(e) => {
                new.status = e.clone();
                Err(e)
            }
        }
    }

    fn calculate_params(&mut self) -> Result<(), LambertSolverStatus> {
        if self.mu <= 0.0 || self.t <= 0.0 {
            return Err(LambertSolverStatus::InvalidInput);
        }
        if self.r1_v.cross(self.r2_v).mag() == 0.0 {
            return Err(LambertSolverStatus::CollinearVectors);
        }
        self.r1 = self.r1_v.mag();
        self.i_r1 = self.r1_v.norm();
        self.r2 = self.r2_v.mag();
        self.i_r2 = self.r2_v.norm();

        self.c_v = self.r2_v - self.r1_v;
        self.c = self.c_v.mag();

        self.s = 0.5 * (self.r1 + self.r2 + self.c);

        self.lambda = (1.0 - self.c / self.s).sqrt();
        self.i_h = self.i_r1.cross(self.i_r2).norm();
        if self.i_h.z < 0.0 {
            self.lambda = -self.lambda;
            self.i_h = -self.i_h;
        }
        self.i_t1 = self.i_h.cross(self.i_r1).norm();
        self.i_t2 = self.i_h.cross(self.i_r2).norm();

        self.t_nd = (2.0 * self.mu / self.s.powf(3.0)).sqrt() * self.t;

        self.find_x()?;
        self.calculate_velocity();

        Ok(())
    }

    fn set_y(&mut self) {
        self.y = (1.0 - self.lambda.powf(2.0) * (1.0 - self.x.powf(2.0))).sqrt();
    }

    fn psi(&self) -> f64 {
        if self.x < 1.0 {
            (self.x * self.y + self.lambda * (1.0 - self.x.powf(2.0))).acos()
        } else if self.x > 1.0 {
            (self.x * self.y - self.lambda * (self.x.powf(2.0) - 1.0)).acosh()
        } else {
            0.0
        }
    }

    fn find_x(&mut self) -> Result<(), LambertSolverStatus> {
        if self.lambda.abs() >= 1.0 {
            return Err(LambertSolverStatus::InternalError);
        }
        if self.t_nd / PI >= 1.0 {
            return Err(LambertSolverStatus::MultiRevolution);
        }

        let tol = 1e-08;

        // T if x = 0
        let t_0 = self.lambda.acos() + self.lambda * (1.0 - self.lambda.powf(2.0)).sqrt();
        // T if x = 1
        let t_1 = 2.0 / 3.0 * (1.0 - self.lambda.powf(3.0));

        // guess x based on non dimentional time-of-flight
        self.x = if self.t_nd >= t_0 {
            (t_0 / self.t_nd).powf(2.0 / 3.0) - 1.0
        } else if self.t_nd < t_1 {
            2.5 * t_1 * (t_1 - self.t_nd) / self.t_nd / (1.0 - self.lambda.powf(5.0)) + 1.0
        } else {
            (t_0 / self.t_nd).powf((t_1 / t_0).log2()) - 1.0
        };
        self.set_y();

        // compute x using householder method

        let mut delta_x: f64 = 2.0 * tol;
        let mut iterations = 10;
        while delta_x.abs() > tol && iterations > 0 {
            let t = 1.0 / (1.0 - self.x.powf(2.0))
                * (self.psi() / (1.0 - self.x.powf(2.0)).abs().sqrt() - self.x
                    + self.lambda * self.y);
            let f_n = t - self.t_nd;
            let f_p = (3.0 * t * self.x - 2.0 + 2.0 * self.lambda.powf(3.0) * self.x / self.y)
                / (1.0 - self.x.powf(2.0));
            let f_pp = (3.0 * t
                + 5.0 * self.x * f_p
                + 2.0 * (1.0 - self.lambda.powf(2.0)) * self.lambda.powf(3.0) / self.y.powf(3.0))
                / (1.0 - self.x.powf(2.0));
            let f_ppp = (7.0 * self.x * f_pp + 8.0 * f_p
                - 6.0 * (1.0 - self.lambda.powf(2.0)) * self.lambda.powf(5.0) * self.x
                    / self.y.powf(5.0))
                / (1.0 - self.x.powf(2.0));
            delta_x = f_n * (f_p.powf(2.0) - f_n * f_pp / 2.0)
                / (f_p * (f_p.powf(2.0) - f_n * f_pp) + f_ppp * f_n.powf(2.0) / 6.0);
            self.x -= delta_x;
            self.set_y();
            iterations -= 1;
        }
        self.status = if iterations > 0 {
            LambertSolverStatus::Converged
        } else {
            LambertSolverStatus::NotConverged
        };
        Ok(())
    }

    fn calculate_velocity(&mut self) {
        let gamma = (self.mu * self.s / 2.0).sqrt();
        let rho = (self.r1 - self.r2) / self.c;
        let sigma = (1.0 - rho.powf(2.0)).sqrt();

        let v_r1 = gamma * (self.lambda * self.y - self.x - rho * (self.lambda * self.y + self.x))
            / self.r1;
        let v_r2 = -gamma * (self.lambda * self.y - self.x + rho * (self.lambda * self.y + self.x))
            / self.r2;
        let v_t1 = gamma * sigma * (self.y + self.lambda * self.x) / self.r1;
        let v_t2 = gamma * sigma * (self.y + self.lambda * self.x) / self.r2;

        self.v1 = v_r1 * self.i_r1 + v_t1 * self.i_t1;
        self.v2 = v_r2 * self.i_r2 + v_t2 * self.i_t2;
    }

    pub fn recalculate_solution(&mut self) {
        self.status = LambertSolverStatus::Initialized;
        match self.calculate_params() {
            Ok(_) => (),
            Err(e) => self.status = e,
        }
    }

    pub fn get_v1(&self) -> Vector3D {
        self.v1
    }

    pub fn get_v2(&self) -> Vector3D {
        self.v2
    }

    pub fn get_orbit(&self) -> Orbit {
        Orbit::from_rv(self.r2_v, self.v2, self.mu)
    }
}

#[cfg(test)]
#[test]
fn test_velocity() {
    let mu = 3.986004e5;
    let r1 = Vector3D {
        x: 5000.0,
        y: 10000.0,
        z: 2100.0,
    };
    let r2 = Vector3D {
        x: -14600.0,
        y: 2500.0,
        z: 7000.0,
    };
    let ls = LambertSolver::new(r1, r2, 3600.0, mu).unwrap();
    let v1 = ls.get_v1();
    let v2 = ls.get_v2();
    let v1_ans = Vector3D {
        x: -5.992494984068112,
        y: 1.925366402070909,
        z: 3.2456379064882404,
    };
    let v2_ans = Vector3D {
        x: -3.31245867404885,
        y: -4.196618846980379,
        z: -0.3852889233316633,
    };
    assert!((v1 - v1_ans).mag() < 0.001);
    assert!((v2 - v2_ans).mag() < 0.001);
}

#[test]
fn test_orbit() {
    let mu = 3.986004e5;
    let r1 = Vector3D {
        x: 5000.0,
        y: 10000.0,
        z: 2100.0,
    };
    let r2 = Vector3D {
        x: -14600.0,
        y: 2500.0,
        z: 7000.0,
    };
    let ls = LambertSolver::new(r1, r2, 3600.0, mu).unwrap();
    let orbit = ls.get_orbit();
    let orbit_ans = Orbit {
        ecc: 0.43348753093376213,
        a: 20002.887624230483,
        inc: 0.8643534138360562,
        lan: 0.7784202841672526,
        argp: 0.5359234295374832,
        nu: 1.5903847969354517,
    };
    assert!((orbit.ecc - orbit_ans.ecc).abs() < 0.001);
    assert!((orbit.a - orbit_ans.a).abs() < 0.001);
    assert!((orbit.inc - orbit_ans.inc).abs() < 0.001);
    assert!((orbit.lan - orbit_ans.lan).abs() < 0.001);
    assert!((orbit.argp - orbit_ans.argp).abs() < 0.001);
    assert!((orbit.nu - orbit_ans.nu).abs() < 0.001);
}
