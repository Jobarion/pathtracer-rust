use crate::geometry::surface::Surface;
use crate::material::material::{Material, Radiator};

pub enum Entity {
    DARK(Box<Surface>, Box<Material>),
    LUMINOUS(Box<Surface>, Box<Radiator>)
}
