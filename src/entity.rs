use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use crate::entity::Entity::{DARK, LUMINOUS};
use crate::geometry::surface::Surface;
use crate::material::material::{Material, Radiator};

pub enum Entity {
    DARK(Box<dyn Surface>, Box<dyn Material>),
    LUMINOUS(Box<dyn Surface>, Box<dyn Radiator>)
}

impl Bounded for Entity {
    fn aabb(&self) -> AABB {
        match self {
            DARK(s, _) => s.aabb(),
            LUMINOUS(s, _) => s.aabb()
        }
    }
}

impl BHShape for Entity {
    fn set_bh_node_index(&mut self, index: usize) {
        match self {
            DARK(s, _) => s.set_bh_node_index(index),
            LUMINOUS(s, _) => s.set_bh_node_index(index)
        }
    }

    fn bh_node_index(&self) -> usize {
        match self {
            DARK(s, _) => s.bh_node_index(),
            LUMINOUS(s, _) => s.bh_node_index()
        }
    }
}
