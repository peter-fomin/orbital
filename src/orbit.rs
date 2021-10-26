#[derive(Default, Debug)]
pub struct Orbit {
    // eccentricity
    pub e: f64,
    // semi-major axis
    pub a: f64,
    // inclination
    pub i: f64,
    // LAN
    pub lan: f64,
    // argument of periapsis
    pub argp: f64,
    // true anomaly
    pub nu: f64,
}
