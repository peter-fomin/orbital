use std::f64::consts::PI;

use super::vectors::Vector3D;

#[derive(Default, Debug)]
pub struct Orbit {
    // eccentricity
    pub ecc: f64,
    // semi-major axis
    pub a: f64,
    // inclination
    pub inc: f64,
    // LAN
    pub lan: f64,
    // argument of periapsis
    pub argp: f64,
    // true anomaly
    pub nu: f64,
}

impl Orbit {
    pub fn from_rv(r: Vector3D, v: Vector3D, mu: f64) -> Self {
        let h = r.cross(v);
        let ecc_v = v.cross(h) / mu - r.norm();
        let n_v = Vector3D::new(-h.y, h.x, 0.0);
        let inc = h.z / h.mag();

        // calculate semi-latus rectum
        let p = h * h / mu;

        // calculate periapse
        // let r_p = p / (1.0 + ecc_v.mag());

        // calculate semi-major axis
        let a = p / (1.0 - ecc_v.mag().powf(2.0));

        // calculate true anomaly
        let nu = if ecc_v.mag() != 0.0 {
            let tmp = (ecc_v * r / r.mag() / ecc_v.mag()).acos();
            if r * v >= 0.0 {
                tmp
            } else {
                2.0 * PI - tmp
            }
        } else if inc != 0.0 {
            let tmp = (n_v * r / r.mag() / n_v.mag()).acos();
            if r.z >= 0.0 {
                tmp
            } else {
                2.0 * PI - tmp
            }
        } else {
            let tmp = (r.x / r.mag()).acos();
            if v.x <= 0.0 {
                tmp
            } else {
                2.0 * PI - tmp
            }
        };

        // calculate lan
        let lan = if n_v.y >= 0.0 {
            (n_v.x / n_v.mag()).acos()
        } else {
            2.0 * PI - (n_v.x / n_v.mag()).acos()
        };

        Self {
            ecc: ecc_v.mag(),
            a: a,
            inc: inc,
            lan: lan,
            argp: (ecc_v * n_v / n_v.mag() / ecc_v.mag()).acos(),
            nu: nu,
        }
    }
}
