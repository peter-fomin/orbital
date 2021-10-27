pub mod vectors;
pub mod orbit;
pub mod lambert;
pub mod data;
pub mod view;

pub use vectors::Vector3D;

pub struct Orbitable {
    pub mass: f64,
    pub radius: f64,
}
