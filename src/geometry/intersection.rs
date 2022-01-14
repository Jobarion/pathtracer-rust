use glam::Vec3;

pub struct Intersection {

    pub position: Vec3,
    pub tangent: Vec3,
    pub normal: Vec3,
    pub distance_squared: f32
}

impl Intersection {
    pub fn new(position: Vec3, normal: Vec3, tangent: Vec3, distance_squared: f32) -> Intersection {
        Intersection {position, normal, tangent, distance_squared}
    }
}