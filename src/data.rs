use druid::{
    text::{
        format::{Formatter, Validation, ValidationError},
        Selection,
    },
    Data, Lens,
};

use crate::LambertSolver;
use crate::Vector3D;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub lambert_problem: LambertSolver,
}

impl AppState {
    pub fn test() -> Self {
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
        Self {
            lambert_problem: ls,
        }
    }

    pub fn initial_earth() -> Self {
        let mu = 3.986004e5;
        let r1 = Vector3D {
            x: 6500.0,
            y: 0.0,
            z: 0.0,
        };
        let r2 = Vector3D {
            x: 0.0,
            y: 6500.0,
            z: 0.0,
        };
        let ls = LambertSolver::new(r1, r2, 3688.0, mu).unwrap();
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
            name: name.to_string(),
            ..Default::default()
        }
    }
}

pub struct FloatFormatter;

impl Formatter<f64> for FloatFormatter {
    fn format(&self, value: &f64) -> String {
        format!("{}", value)
    }

    fn validate_partial_input(&self, _input: &str, _sel: &Selection) -> Validation {
        Validation::success()
    }

    fn value(&self, input: &str) -> Result<f64, ValidationError> {
        input.parse().map_err(ValidationError::new)
    }
}
