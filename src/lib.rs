pub mod vectors;
pub mod orbit;
pub mod lambert;

pub use vectors::Vector3D;

pub struct Orbitable {
    pub mass: f64,
    pub radius: f64,
}
