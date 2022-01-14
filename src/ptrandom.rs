use rand::Rng;
use glam::Vec3;

pub fn get_unit() -> f32 {
    rand::thread_rng().gen()
}

pub fn get_bi_unit() -> f32 {
    rand::thread_rng().gen::<f32>() * 2.0 - 1.0
}

pub fn get_longitude() -> f32 {
    rand::thread_rng().gen::<f32>() * std::f32::consts::PI * 2.0
}

pub fn get_wavelength() -> f32 {
    get_unit() * 400.0 + 300.0
}

pub fn get_hemisphere_vector() -> Vec3 {
    let phi = get_longitude();
    let rq = get_unit();
    let r = rq.sqrt();
    Vec3::new(phi.cos() * r, phi.sin() * r, (1.0 - rq).sqrt())
}

