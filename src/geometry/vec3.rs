use super::quaternion::Quaternion;

#[derive(Clone, PartialEq, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

pub const ORIGIN: &Vec3 = &Vec3 { x: 0.0, y: 0.0, z: 0.0 };

impl Vec3 {

    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x, y, z}
    }

    pub fn add(&self, vec: &Vec3) -> Vec3 {
        Vec3 { x: self.x + vec.x, y: self.y + vec.y, z: self.z + vec.z}
    }

    pub fn subtract(&self, vec: &Vec3) -> Vec3 {
        Vec3 { x: self.x - vec.x, y: self.y - vec.y, z: self.z - vec.z}
    }

    pub fn scale(&self, f: f64) -> Vec3 {
        Vec3 { x: self.x * f, y: self.y * f, z: self.z * f}
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(&self) -> Vec3 {
        self.scale(1.0 / self.length())
    }

    pub fn dot(&self, vec: &Vec3) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z
    }

    pub fn cross(&self, vec: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * vec.z - self.z * vec.y,
            y: self.z * vec.x - self.x * vec.z,
            z: self.x * vec.y - self.y * vec.x
        }
    }

    pub fn reflect(&self, vec: &Vec3) -> Vec3 {
        self.subtract(&vec.scale(vec.dot(self) * 2.0))
    }

    pub fn rotate(&self, q: &Quaternion) -> Vec3 {
        let p = Quaternion::new(self.x, self.y, self.z, 0.0);
        let r = q.multiply(&p).multiply(&q.conjugate());
        Vec3 { x: r.x, y: r.y, z: r.z }
    }

    pub fn rotate_towards(&self, v: &Vec3) -> Vec3 {
        let dot = v.z;
        if dot > 0.9999 {
            return self.clone();
        }
        else if dot < -0.9999 {
            return Vec3::new(self.x, self.y, -self.z);
        }

        let a1 = Vec3::new(0.0, 0.0, 1.0).cross(v).normalize();
        let a2 = a1.cross(v).normalize();
        return a1.scale(self.x).add(&a2.scale(self.y)).add(&v.scale(self.z));
    }
}