use super::vec3::Vec3;

pub trait Volume {

    fn is_inside(&self, vec: &Vec3) -> bool;
}