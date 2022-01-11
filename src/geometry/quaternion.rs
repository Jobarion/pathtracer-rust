#[derive(Clone)]
pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Quaternion {

    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Quaternion {
        Quaternion { x, y, z, w}
    }

    pub fn rotation(x: f64, y: f64, z: f64, angle: f64) -> Quaternion {
        Quaternion {
            x: (angle * 0.5).sin() * x,
            y: (angle * 0.5).sin() * y,
            z: (angle * 0.5).sin() * z,
            w: (angle * 0.5).cos()
        }
    }

    pub fn conjugate(&self) -> Quaternion {
        Quaternion { x: -self.x, y: -self.y, z: -self.z, w: self.w }
    }

    pub fn add(&self, quad: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.x + quad.x,
            y: self.y + quad.y,
            z: self.z + quad.z,
            w: self.w + quad.w
        }
    }

    pub fn subtract(&self, quad: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.x - quad.x,
            y: self.y - quad.y,
            z: self.z - quad.z,
            w: self.w - quad.w
        }
    }

    pub fn scale(&self, f: f64) -> Quaternion {
        Quaternion {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
            w: self.w * f
        }
    }

    pub fn multiply(&self, q: &Quaternion) -> Quaternion {
        Quaternion {
            x: self.w * q.x + self.x * q.w + self.y * q.z - self.z * q.y,
            y: self.w * q.y - self.x * q.z + self.y * q.w + self.z * q.x,
            z: self.w * q.z + self.x * q.y - self.y * q.x + self.z * q.w,
            w: self.w * q.w - self.x * q.x - self.y * q.y - self.z * q.z
        }
    }
}