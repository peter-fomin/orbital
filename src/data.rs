use druid::Data;

use super::lambert::LambertSolver;
use super::vectors::Vector3D;

#[derive(Clone, Data)]
pub struct AppState {
    lambert_problem: LambertSolver,
}

impl AppState {
    pub fn test() -> Self {
        let mu = 3.986004e5;
        let r1 = Vector3D {
            x: 5000.0,
            y: 10000.0,
            z: 2100.0
        };
        let r2 = Vector3D {
            x: -14600.0,
            y: 2500.0,
            z: 7000.0
        };
        let ls = LambertSolver::new(r1, r2, 3600.0, mu);
        Self {
            lambert_problem: ls,
        }
    }
}

#[derive(Default, Clone, Data)]
pub struct VectorItem {
    name: String,
    value: Vector3D,
}

impl VectorItem {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            ..Default::default()
        }
    }
}