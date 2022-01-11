use super::vec3::Vec3;

pub struct Ray {
    pub position: Vec3,
    pub direction: Vec3,
    pub wavelength: f64,
    pub strength: f64
}

impl Ray {

    pub fn new(position: Vec3, direction: Vec3, wavelength: f64, strength: f64) -> Ray {
        Ray { position, direction, wavelength, strength }
    }
}