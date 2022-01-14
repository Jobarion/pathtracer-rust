mod geometry;
mod material;
mod ptrandom;
mod scene;
mod entity;
mod tracer;
mod plotter;

use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;
use bvh::aabb::Bounded;
use ply_rs::{parser, ply};
use rand::Rng;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use rtrace::tracer::Photon;
use crate::entity::Entity;
use crate::geometry::circle::Circle;
use crate::geometry::mesh;
use crate::geometry::mesh::Triangle;
use glam::{Quat, Vec3, Vec4};
use crate::material::glossy::GlossyMaterial;
use crate::scene::{Camera, Scene};
use crate::geometry::plane::Plane;
use crate::geometry::sphere::Sphere;
use crate::geometry::surface::Surface;
use crate::material::black_body_radiator::BlackBodyRadiator;
use crate::material::diffuse::{DiffuseGrayMaterial, SimpleDiffuseColoredMaterial};
use crate::material::glass::{BandPassColoredGlassMaterial, GaussianColoredGlassMaterial, GlassMaterial};
use crate::material::spectrum_radiator::SpectrumRadiator;
use crate::plotter::Plotter;
use crate::tracer::RenderIterator;

fn main() {

    let scene = create_scene_model();

    let width = 512_u16;
    let height = 512_u16;

    let mut plotter = Plotter::new(width, height);

    let mut ray_count = 0;
    let rays_per_pixel = 50;
    let start = Instant::now();
    loop {
        plotter.merge(render_scene_parallel(&scene, width, height, rays_per_pixel));
        let rgb_data = plotter.tone_map();
        ray_count += rays_per_pixel as u128 * width as u128 * height as u128;

        let data = rgb_data.
            iter()
            .fold(Vec::with_capacity(rgb_data.len() * 3), |mut array, c| {
                array.push(c.0);
                array.push(c.1);
                array.push(c.2);
                array
            });

        println!("Rendered {} rays in {} seconds ({}/s)", ray_count, start.elapsed().as_secs(), ray_count / start.elapsed().as_secs() as u128);

        let file_name = "rendered.png";
        let path = Path::new(file_name);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, width as u32, height as u32);
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(data.as_slice());
    }
}

fn render_scene_parallel(scene: &Scene, width: u16, height: u16, rays_per_pixel: u32) -> Plotter {
    let slice_count = 1024;
    let rays_per_slice = (rays_per_pixel as u128 * height as u128 * width as u128) / slice_count as u128;
    println!("Rays per slice {}", rays_per_slice);
    (0..slice_count)
        .into_par_iter()
        .map(|slice| RenderIterator::new_sliced(scene, -1.0, 1.0, -1.0 + (2.0 / slice_count as f32) * slice as f32, 1.0 - (2.0 / slice_count as f32) * (slice_count - slice - 1) as f32))
        .map(|riter| riter
            .take(rays_per_slice as usize)
            .fold(Plotter::new(width, height), |mut acc, val| {
                acc.plot_photon(val);
                return acc;
            })
        )
        .reduce(|| Plotter::new(width, height), |mut acc, val| {
            acc.merge(val);
            return acc;
        })
}

fn create_scene_color_filter() -> Scene {
    let light_1 = Entity::LUMINOUS(Box::new(Circle::new(Vec3::new(0.0, 10.0, 0.0), Vec3::new(0.0, 1.0, 0.0), 5.0)), Box::new(SpectrumRadiator::new(300.0, 800.0)));
    let filter_green = Entity::DARK(Box::new(Circle::new(Vec3::new(0.0_f32.to_radians().sin(), 0.1, 0.0_f32.to_radians().cos()), Vec3::new(0.0, 1.0, 0.0), 1.5)), Box::new(BandPassColoredGlassMaterial::new(500.0, 750.0)));
    let filter_yellow = Entity::DARK(Box::new(Circle::new(Vec3::new(120.0_f32.to_radians().sin(), 0.0, 120.0_f32.to_radians().cos()), Vec3::new(0.0, 1.0, 0.0), 1.5)), Box::new(BandPassColoredGlassMaterial::new(570.0, 585.0)));
    let filter_red = Entity::DARK(Box::new(Circle::new(Vec3::new(240.0_f32.to_radians().sin(), -0.1, 240.0_f32.to_radians().cos()), Vec3::new(0.0, 1.0, 0.0), 1.5)), Box::new(BandPassColoredGlassMaterial::new(525.0, 540.0)));

    let entities = vec![light_1, filter_green, filter_yellow, filter_red];

    let camera = Camera::new(
        Vec3::new(0.0, -5.0, 0.0),
        Quat::from_vec4(Vec4::new(0.0, 1.0, 0.0, 0.0)).normalize(),
        std::f32::consts::PI * 0.35,
        4.0,
        f32::MAX,
        0.01
    );
    Scene::new(entities, camera)
}

fn create_scene_simple() -> Scene {
    let sun1 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5.0)), Box::new(BlackBodyRadiator::new(6800.0, 8.0)));
    let back = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 5.5, 0.0), Vec3::new(0.0, 1.0, 0.0))), Box::new(GlossyMaterial::new(0.8, Box::new(DiffuseGrayMaterial::new(1.0)))));


    let entities = vec![sun1, back];
    let camera = Camera::new(
        Vec3::new(0.0, -9.0, 0.0),
        Quat::from_vec4(Vec4::new(0.0, 1.0, 0.0, 0.0)).normalize(),
        std::f32::consts::PI * 0.35,
        4.0,
        f32::MAX,
        0.01
    );
    Scene::new(entities, camera)
}


fn create_scene_model() -> Scene {
    let top = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 1.0))), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));
    let bottom = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, 1.0))), Box::new(DiffuseGrayMaterial::new(0.8)));
    // let front = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, -10.0, 0.0), Vec3::new(0.0, 1.0, 0.0))), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 500.0, 10.0)));
    let front = Entity::LUMINOUS(Box::new(Plane::new(Vec3::new(0.0, -10.0, 0.0), Vec3::new(0.0, 1.0, 0.0))), Box::new(BlackBodyRadiator::new(7000.0, 1.0)));
    let back = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 10.0, 0.0), Vec3::new(0.0, 1.0, 0.0))), Box::new(GlossyMaterial::new(0.8, Box::new(DiffuseGrayMaterial::new(1.0)))));
    let left = Entity::DARK(Box::new(Plane::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0))), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 400.0, 20.0)));
    let right = Entity::DARK(Box::new(Plane::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0))), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 600.0, 40.0)));

    // let sun1 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 9.0), 1.0)), Box::new(BlackBodyRadiator::new(6800.0, 1.0)));
    // let sun2 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(4.0, 5.0, -3.0), 1.0)), Box::new(BlackBodyRadiator::new(7000.0, 1.5)));
    let sun3 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(-4.0, 5.0, -5.0), 2.0)), Box::new(BlackBodyRadiator::new(9000.0, 6.0)));

    // let mirror_sphere1 = Entity::DARK(Box::new(Sphere::new(Vec3::new(3.0, 2.0, 7.0), 1.8)), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));
    // let mirror_sphere2 = Entity::DARK(Box::new(Sphere::new(Vec3::new(0.0, 4.0, 8.0), 2.0)), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));
    // let glass_sphere_1 = Entity::DARK(Box::new(Sphere::new(Vec3::new(0.6, -4.0, 4.5), 0.9)), Box::new(GlassMaterial));
    // let colored_sphere1 = Entity::DARK(Box::new(Sphere::new(Vec3::new(7.0, 5.0, 5.0), 1.99)), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 580.0, 50.0)));
    // let glass_coating = Entity::DARK(Box::new(Sphere::new(Vec3::new(7.0, 5.0, 5.0), 2.00)), Box::new(GlassMaterial));
    // let glass_sphere_3 = Entity::DARK(Box::new(Sphere::new(Vec3::new(-4.0, 6.0, 2.0), 3.00)), Box::new(GlassMaterial));

    // let triangle_mirror = Entity::DARK(Box::new(Triangle::new(Vec3::new(-3.0, 9.9, 9.0), Vec3::new(3.0, 9.9, 9.0), Vec3::new(0.0, 9.9, 6.0))), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));

    // let path = "tetrahedron.ply";
    // let path = "bun_zipper.ply";
    let path = "lucy.ply";
    let mut f = std::fs::File::open(path).unwrap();
    let mut bunny_mesh = mesh::read_ply(f);
    println!("Loaded mesh");
    let diameter = (bunny_mesh.aabb().max - bunny_mesh.aabb().min).length();
    let target_diameter = 12.0;
    println!("Scale (factor {})", target_diameter / diameter);
    bunny_mesh = bunny_mesh.scale(target_diameter / diameter);
    println!("Scaled");
    bunny_mesh = bunny_mesh.rotate(Quat::from_rotation_x(180.0_f32.to_radians()));
    println!("Rotated");
    let center = bunny_mesh.get_center();
    let t = Vec3::new(-center.x, center.y, 10.0 - bunny_mesh.aabb().max.z);
    bunny_mesh = bunny_mesh.translate(t);
    println!("Translated");

    let bunny = Entity::DARK(Box::new(bunny_mesh), Box::new(DiffuseGrayMaterial::new(0.9)));
    // let bunny = Entity::DARK(Box::new(bunny_mesh), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));

    let entities = vec![top, bottom, front, back, left, right, sun3, bunny];

    let camera = Camera::new(
        Vec3::new(0.0, -8.0, -5.0),
        Quat::from_vec4(Vec4::new(0.0, 10.0, 1.0, 0.0)).normalize(),
        std::f32::consts::PI * 0.35,
        4.0,
        f32::MAX,
        0.01
    );
    Scene::new(entities, camera)
}

fn create_scene_box() -> Scene {
    let top = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 1.0))), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));
    let bottom = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 0.0, 1.0))), Box::new(DiffuseGrayMaterial::new(0.7)));
    let front = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, -10.0, 0.0), Vec3::new(0.0, 1.0, 0.0))), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 500.0, 10.0)));
    // let front = Entity::LUMINOUS(Box::new(Plane::new(Vec3::new(0.0, -10.0, 0.0), Vec3::new(0.0, 1.0, 0.0))), Box::new(BlackBodyRadiator::new(3000.0, 1.0)));
    let back = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 10.0, 0.0), Vec3::new(0.0, 1.0, 0.0))), Box::new(GlossyMaterial::new(0.8, Box::new(DiffuseGrayMaterial::new(0.7)))));
    let left = Entity::DARK(Box::new(Plane::new(Vec3::new(10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0))), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 400.0, 20.0)));
    let right = Entity::DARK(Box::new(Plane::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0))), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 600.0, 40.0)));

    let sun1 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 9.0), 1.0)), Box::new(BlackBodyRadiator::new(6800.0, 1.0)));
    let sun2 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(4.0, 5.0, -3.0), 1.0)), Box::new(BlackBodyRadiator::new(7000.0, 1.5)));
    let sun3 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(-4.0, 5.0, -3.0), 3.0)), Box::new(BlackBodyRadiator::new(9000.0, 1.0)));

    let mirror_sphere1 = Entity::DARK(Box::new(Sphere::new(Vec3::new(3.0, 2.0, 7.0), 1.8)), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));
    let mirror_sphere2 = Entity::DARK(Box::new(Sphere::new(Vec3::new(0.0, 4.0, 8.0), 2.0)), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));
    let glass_sphere_1 = Entity::DARK(Box::new(Sphere::new(Vec3::new(0.6, -4.0, 4.5), 0.9)), Box::new(GlassMaterial));
    let colored_sphere1 = Entity::DARK(Box::new(Sphere::new(Vec3::new(7.0, 5.0, 5.0), 1.99)), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 580.0, 50.0)));
    let glass_coating = Entity::DARK(Box::new(Sphere::new(Vec3::new(7.0, 5.0, 5.0), 2.00)), Box::new(GlassMaterial));
    let glass_sphere_3 = Entity::DARK(Box::new(Sphere::new(Vec3::new(-4.0, 6.0, 2.0), 3.00)), Box::new(GlassMaterial));

    // let triangle_mirror = Entity::DARK(Box::new(Triangle::new(Vec3::new(-3.0, 9.9, 9.0), Vec3::new(3.0, 9.9, 9.0), Vec3::new(0.0, 9.9, 6.0))), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));

    // let path = "tetrahedron.ply";
    // let path = "bun_zipper.ply";
    let path = "lucy.ply";
    let mut f = std::fs::File::open(path).unwrap();
    let mut bunny_mesh = mesh::read_ply(f);
    println!("Loaded mesh");
    let diameter = (bunny_mesh.aabb().max - bunny_mesh.aabb().min).length();
    let target_diameter = 6.0;
    println!("Scale (factor {})", target_diameter / diameter);
    bunny_mesh = bunny_mesh.scale(target_diameter / diameter);
    println!("Scaled");
    bunny_mesh = bunny_mesh.rotate(Quat::from_rotation_x(270.0_f32.to_radians()) * Quat::from_rotation_y(180.0_f32.to_radians()));
    println!("Rotated");
    let t = Vec3::new(-4.0, 1.0, 10.0 - bunny_mesh.aabb().max.z);
    bunny_mesh = bunny_mesh.translate(t);
    println!("Translated");

    let bunny = Entity::DARK(Box::new(bunny_mesh), Box::new(SimpleDiffuseColoredMaterial::new(0.8, 550.0, 30.0)));
    // let bunny = Entity::DARK(Box::new(bunny_mesh), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));

    let entities = vec![top, bottom, front, back, left, right, sun1, sun2, sun3, mirror_sphere1, mirror_sphere2, glass_sphere_1, glass_coating, colored_sphere1, glass_sphere_3, bunny];

    let camera = Camera::new(
        Vec3::new(0.0, -9.0, -4.0),
        Quat::from_vec4(Vec4::new(0.0, 10.0, 3.0, 0.0)).normalize(),
        std::f32::consts::PI * 0.35,
        4.0,
        f32::MAX,
        0.01
    );
    Scene::new(entities, camera)
}