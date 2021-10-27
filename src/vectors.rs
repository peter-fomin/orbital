// Basic 3D Vector functions
use std::ops::{Add, Sub, Neg, Mul, Div};

use druid::{Data, Lens};

#[derive(Default, Debug, Copy, Clone, PartialEq, Data, Lens)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Add for Vector3D {
    type Output = Self;
    
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vector3D {
    type Output = Self;
    
    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector3D {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<Vector3D> for f64 {
    // Scalar - Vector product
    type Output = Vector3D;

    fn mul(self, rhs: Vector3D) -> Vector3D {
        Vector3D {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<f64> for Vector3D {
    // Vector - Scalar product
    type Output = Self;
    
    fn mul(self, rhs: f64) -> Self {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul for Vector3D {
    type Output = f64;

    fn mul(self, rhs: Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}
    
impl Div<f64> for Vector3D {
    // Divide Vector by Scalar
    type Output = Self;
    
    fn div(self, rhs: f64) -> Self {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        // Standart constructor
        Self {
            x,
            y,
            z,
        }
    }

    pub fn mag(&self) -> f64 {
        // Vector magnitude
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn norm(&self) -> Self {
        // Normalized vector
        *self / self.mag()
    }

    pub fn cross(self, rhs: Vector3D) -> Vector3D {
        // Vector cross product
        Self{
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}


#[cfg(test)]
  
#[test]
fn test_add() {
    assert_eq!(
        Vector3D::new(1.0, -1.0, 0.0) + Vector3D::new(2.0, 2.0, 2.0),
        Vector3D::new(3.0, 1.0, 2.0)
    );
}

#[test]
fn test_sub() {
    assert_eq!(
        Vector3D::new(1.0, -1.0, 0.0) - Vector3D::new(2.0, 2.0, 2.0),
        Vector3D::new(-1.0, -3.0, -2.0)
    );
}

#[test]
fn test_mul() {
    assert_eq!(
        3.0 * Vector3D::new(1.0, -1.0, 0.0),
        Vector3D::new(3.0, -3.0, 0.0)
    );
    assert_eq!(
        Vector3D::new(1.0, -1.0, 0.0) * 3.0,
        Vector3D::new(3.0, -3.0, 0.0)
    );
}

#[test]
fn test_div() {
    assert_eq!(
        Vector3D::new(3.0, -3.0, 0.0) / 3.0,
        Vector3D::new(1.0, -1.0, 0.0)
    );
}
