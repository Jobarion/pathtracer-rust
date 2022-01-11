use super::vec3::Vec3;

pub struct Intersection {

    pub position: Vec3,
    pub tangent: Vec3,
    pub normal: Vec3,
    pub distance: f64
}

impl Intersection {
    pub fn new(position: Vec3, normal: Vec3, tangent: Vec3, distance: f64) -> Intersection {
        Intersection {position, normal, tangent, distance}
    }
}