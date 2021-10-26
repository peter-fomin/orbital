use orbital::Vector3D;
use orbital::lambert::LambertSolver;

fn main() {
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
    let (v1, v2) = ls.get_velocity();
    let orbit = ls.get_orbit();
    println!("{:?}", v1);
    println!("{:?}", v2);
    println!("{:?}", orbit);

}