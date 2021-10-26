pub mod vectors;
pub mod lambert;

pub use vectors::Vector3D;

pub struct Orbit {
    pub a: f64,
}

pub struct Orbitable {
    pub mass: f64,
    pub radius: f64,
}
