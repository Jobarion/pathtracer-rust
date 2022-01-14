use glam::Vec3;

pub fn reflect(a: Vec3, b: Vec3) -> Vec3 {
    a - b * (b.dot(a) * 2.0)
}

pub fn rotate_towards(a: Vec3, b: Vec3) -> Vec3 {
    let dot = b.z;
    if dot > 0.9999 {
        return a;
    }
    else if dot < -0.9999 {
        return Vec3::new(a.x, a.y, -a.z);
    }

    let a1 = Vec3::new(0.0, 0.0, 1.0).cross(b).normalize();
    let a2 = a1.cross(b).normalize();
    return a1 * a.x + a2 * a.y + b * a.z;
}