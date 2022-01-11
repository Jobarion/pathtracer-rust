use rand::Rng;
use crate::geometry::vec3::Vec3;

pub fn get_unit() -> f64 {
    rand::thread_rng().gen()
}

pub fn get_bi_unit() -> f64 {
    rand::thread_rng().gen::<f64>() * 2.0 - 1.0
}

pub fn get_longitude() -> f64 {
    rand::thread_rng().gen::<f64>() * std::f64::consts::PI * 2.0
}

pub fn get_wavelength() -> f64 {
    get_unit() * 400.0 + 300.0
}

pub fn get_hemisphere_vector() -> Vec3 {
    let phi = get_longitude();
    let rq = get_unit();
    let r = rq.sqrt();
    Vec3::new(phi.cos() * r, phi.sin() * r, (1.0 - rq).sqrt())
}

