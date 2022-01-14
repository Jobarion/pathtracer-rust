use std::fs::File;
use bvh::aabb::{AABB, Bounded};
use bvh::bounding_hierarchy::BHShape;
use bvh::bvh::BVH;
use bvh::{Point3, Vector3};
use ply_rs::{parser, ply};
use ply_rs::ply::Property;
use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::surface::Surface;
use glam::{Quat, Vec3};

pub struct Mesh {
    bvh: Box<BVH>,
    triangles: Vec<Triangle>,
    aabb: AABB,
    node_index: usize
}

impl Mesh {
    pub fn new(mut triangles: Vec<Triangle>) -> Mesh {
        let bvh = BVH::build(&mut triangles);
        let (min, max) = triangles.iter()
            .fold((Point3::new(f32::MAX, f32::MAX, f32::MAX), Point3::new(f32::MIN, f32::MIN, f32::MIN)), |mut acc, val| {
                let t_aabb = val.aabb();
                acc.0 = Point3::new(acc.0.x.min(t_aabb.min.x), acc.0.y.min(t_aabb.min.y), acc.0.z.min(t_aabb.min.z));
                acc.1 = Point3::new(acc.1.x.max(t_aabb.max.x), acc.1.y.max(t_aabb.max.y), acc.1.z.max(t_aabb.max.z));
                acc
            });
        Mesh { bvh: Box::new(bvh), triangles, aabb: AABB::with_bounds(min, max), node_index: 0 }
    }

    pub fn get_center(&self) -> Vec3 {
        Vec3::new((self.aabb.min.x + self.aabb.max.x) as f32 / 2.0, (self.aabb.min.y + self.aabb.max.y) as f32 / 2.0, (self.aabb.min.z + self.aabb.max.z) as f32 / 2.0)
    }

    pub fn rotate(self, rotation: Quat) -> Mesh {
        let mid = self.get_center();
        let triangles = self.translate(mid * -1.0)
            .triangles
            .into_iter()
            .map(|x| Triangle::new(rotation.mul_vec3(x.a), rotation.mul_vec3(x.b), rotation.mul_vec3(x.c)))
            .collect();
        Mesh::new(triangles).translate(mid)
    }

    pub fn scale(self, factor: f32) -> Mesh {
        let mid = self.get_center();
        let triangles = self.translate(mid * -1.0)
            .triangles
            .into_iter()
            .map(|x| Triangle::new(x.a * factor, x.b * factor, x.c * factor))
            .collect();
        Mesh::new(triangles)
    }

    pub fn translate(self, translation: Vec3) -> Mesh {
        let triangles = self.triangles
            .into_iter()
            .map(|x| Triangle::new(x.a + translation, x.b + translation, x.c + translation))
            .collect();
        Mesh::new(triangles)
    }
}

impl Bounded for Mesh {
    fn aabb(&self) -> AABB {
        self.aabb
    }
}

impl BHShape for Mesh {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

impl Surface for Mesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let bvh_ray = bvh::ray::Ray::new(Point3::new(ray.position.x, ray.position.y, ray.position.z), Vector3::new(ray.direction.x, ray.direction.y, ray.direction.z));
        let candidates = self.bvh.traverse(&bvh_ray, &self.triangles);

        let mut min_distance = f32::INFINITY;
        let mut result: Option<Intersection> = None;
        for e in candidates {
            let intersection = e.intersect(ray);
            if let Some(i) = intersection {
                let dist = i.distance_squared;
                if dist < min_distance {
                    result = Some(i);
                    min_distance = dist;
                }
            }
        }
        result
    }
}

#[derive(Debug)]
pub struct Triangle {
    a: Vec3,
    b: Vec3,
    c: Vec3,
    normal: Vec3,
    node_index: usize
}

impl Triangle {
    pub fn new(a: Vec3, b: Vec3, c: Vec3) -> Triangle {
        let normal = (b - a).cross(c - a);
        Triangle { a, b, c, normal, node_index: 0 }
    }
}

impl Bounded for Triangle {
    fn aabb(&self) -> AABB {
        let min = Point3::new(self.a.x.min(self.b.x).min(self.c.x) as f32, self.a.y.min(self.b.y).min(self.c.y) as f32, self.a.z.min(self.b.z).min(self.c.z) as f32);
        let max = Point3::new(self.a.x.max(self.b.x).max(self.c.x) as f32, self.a.y.max(self.b.y).max(self.c.y) as f32, self.a.z.max(self.b.z).max(self.c.z) as f32);
        AABB::with_bounds(min, max)
    }
}

impl BHShape for Triangle {
    fn set_bh_node_index(&mut self, index: usize) {
        self.node_index = index;
    }

    fn bh_node_index(&self) -> usize {
        self.node_index
    }
}

struct PlyTriangleReader {
    pub vertices: Vec<Vec3>,
    pub faces: Vec<Triangle>
}

pub fn read_ply(mut f: File) -> Mesh {
    let p = parser::Parser::<ply::DefaultElement>::new();

    let ply = p.read_ply(&mut f);

    let triangles = ply.unwrap()
        .payload
        .iter()
        .fold(PlyTriangleReader { vertices: vec![], faces: vec![] }, |mut acc, val| {
            match val {
                ( a, d) if a == "vertex" => {
                    acc.vertices = d.iter()
                        .fold(acc.vertices, |mut vacc, vval| {
                            let x= read_num_property(vval.get("x").unwrap());
                            let y= read_num_property(vval.get("y").unwrap());
                            let z= read_num_property(vval.get("z").unwrap());

                            vacc.push(Vec3::new(x, y, z));
                            return vacc;
                        });
                },
                ( a, d) if a == "face" => {
                    let vertices = &acc.vertices;
                    acc.faces = d.iter()
                        .fold(acc.faces, |mut vacc, vval| {
                            if let Property::ListInt(vec) = vval.get("vertex_indices").unwrap() {
                                assert_eq!(3, vec.len(), "We only support triangular faces");
                                let triangle = Triangle::new(
                                    vertices.get(*vec.get(0).unwrap() as usize).unwrap().clone(),
                                    vertices.get(*vec.get(1).unwrap() as usize).unwrap().clone(),
                                    vertices.get(*vec.get(2).unwrap() as usize).unwrap().clone()
                                );
                                vacc.push(triangle);
                                vacc
                            } else {
                                panic!()
                            }
                        });
                }
                _ => panic!()
            }
            acc
        })
        .faces;
    Mesh::new(triangles)
}

fn read_num_property(prop: &Property) -> f32{
    match prop {
        Property::Double(x) => *x as f32,
        Property::Float(x) => *x,
        Property::Int(x) => *x as f32,
        Property::UInt(x) => *x as f32,
        _ => panic!()
    }
}


impl Surface for Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {

        let e1 = self.b - self.a;
        let e2 = self.c - self.a;
        let pvec = ray.direction.cross(e2);
        let det = e1.dot(pvec);
        if det.abs() < f32::MIN {
            return None;
        }

        let inv_det = 1.0 / det;
        let tvec = ray.position - self.a;
        let u = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 {
            return None;
        }

        let qvec = tvec.cross(e1);
        let v = ray.direction.dot(qvec) * inv_det;
        if v < 0.0 || (u + v) > 1.0 {
            return None;
        }

        let distance = e2.dot(qvec) * inv_det;

        if distance > f32::EPSILON {
            let p = ray.position + ray.direction * distance;

            Some(Intersection::new(p, self.normal.clone(), Vec3::new(0.0, 0.0, 0.0), distance * distance))
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::geometry::ray::Ray;
    use crate::geometry::mesh::Triangle;
    use crate::geometry::surface::Surface;
    use glam::{Quat, Vec3, Vec4};
    use crate::geometry::util;

    #[test]
    fn triangle_intersection_1() {
        let s = Triangle::new(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        println!("{:?}", s.normal);
        let r = Ray::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(false),
            Some(hit) => assert_eq!(hit.position, Vec3::new(0.0, 0.0, 0.0))
        }
    }

    #[test]
    fn triangle_intersection_2() {
        let s = Triangle::new(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        println!("{:?}", s.normal);
        let r = Ray::new(Vec3::new(0.0, -1.0, 0.0), Vec3::new(0.0, -1.0, 0.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(true),
            Some(hit) => assert!(false)
        }
    }

    #[test]
    fn triangle_intersection_3() {
        let s = Triangle::new(Vec3::new(-1.0, 0.0, -1.0), Vec3::new(1.0, 0.0, -1.0), Vec3::new(0.0, 0.0, 1.0));
        println!("{:?}", s.normal);
        let r = Ray::new(Vec3::new(1.0, -1.0, 1.0), Vec3::new(0.0, 1.0, 0.0), 1.0, 1.0);
        let i = s.intersect(&r);
        match i {
            None => assert!(true),
            Some(hit) => assert!(false)
        }
    }
}