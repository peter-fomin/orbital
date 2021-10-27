use druid::{Data, Lens};

use super::vectors::Vector3D;
use super::lambert::LambertSolver;

#[derive(Data)]
struct LambertWidget {

    lambert_solver: LambertSolver
}