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
use rand::Rng;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use rtrace::tracer::Photon;
use crate::entity::Entity;
use crate::geometry::vec3::Vec3;
use crate::material::glossy::GlossyMaterial;
use crate::scene::{Camera, Scene};
use crate::geometry::plane::Plane;
use crate::geometry::quaternion::Quaternion;
use crate::geometry::sphere::Sphere;
use crate::material::black_body_radiator::BlackBodyRadiator;
use crate::material::diffuse::{DiffuseGrayMaterial, SimpleDiffuseColoredMaterial};
use crate::material::glass::GlassMaterial;
use crate::plotter::Plotter;
use crate::tracer::RenderIterator;

fn main() {
    let scene = create_scene_box();

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

        // let file_name = format!("rendered_{}.png", ray_count);
        let file_name = "rendered.png";
        let path = Path::new(file_name);
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, width as u32, height as u32); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgb);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().unwrap();

        writer.write_image_data(data.as_slice());
    }

    println!("Done");
}

fn render_scene_parallel(scene: &Scene, width: u16, height: u16, rays_per_pixel: u32) -> Plotter {
    let slice_count = 1024;
    let rays_per_slice = (rays_per_pixel as u128 * height as u128 * width as u128) / slice_count as u128;
    println!("Rays per slice {}", rays_per_slice);
    (0..slice_count)
        .into_par_iter()
        .map(|slice| RenderIterator::new_sliced(scene, -1.0, 1.0, -1.0 + (2.0 / slice_count as f64) * slice as f64, 1.0 - (2.0 / slice_count as f64) * (slice_count - slice - 1) as f64))
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

fn create_scene_simple() -> Scene {
    let sun1 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 0.0), 5.0)), Box::new(BlackBodyRadiator::new(6800.0, 8.0)));

    let entities = vec![sun1];
    let camera = Camera::new(
        Vec3::new(0.0, -9.0, 0.0),
        Quaternion::new(0.0, 1.0, 0.0, 0.0),
        std::f64::consts::PI * 0.35,
        4.0,
        f64::MAX,
        0.01
    );
    Scene::new(entities, camera)
}

fn create_scene_box() -> Scene {
    let top = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 1.0))), Box::new(GlossyMaterial::new(0.0, Box::new(DiffuseGrayMaterial::new(1.0)))));
    let bottom = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 0.0, 10.0),  Vec3::new(0.0, 0.0, 1.0))), Box::new(DiffuseGrayMaterial::new(0.8)));
    let front = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, -10.0, 0.0), Vec3::new(0.0, 1.0, 0.0))), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 500.0, 10.0)));
    let back = Entity::DARK(Box::new(Plane::new(Vec3::new(0.0, 10.0, 0.0),  Vec3::new(0.0, 1.0, 0.0))), Box::new(GlossyMaterial::new(0.8, Box::new(DiffuseGrayMaterial::new(1.0)))));
    let left = Entity::DARK(Box::new(Plane::new(Vec3::new(10.0, 0.0, 0.0),  Vec3::new(1.0, 0.0, 0.0))), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 400.0, 20.0)));
    let right = Entity::DARK(Box::new(Plane::new(Vec3::new(-10.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0))), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 600.0, 40.0)));

    let sun1 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(0.0, 0.0, 9.0), 1.0)), Box::new(BlackBodyRadiator::new(6800.0, 8.0)));
    let sun2 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(7.0, 5.0, 5.0), 2.0)), Box::new(BlackBodyRadiator::new(4000.0, 150.0)));
    let sun3 = Entity::LUMINOUS(Box::new(Sphere::new(Vec3::new(-4.0, 5.0, -3.0), 3.0)), Box::new(BlackBodyRadiator::new(9000.0, 6.0)));

    let colored_sphere1 = Entity::DARK(Box::new(Sphere::new(Vec3::new(7.0, 5.0, 5.0), 2.0)), Box::new(SimpleDiffuseColoredMaterial::new(1.0, 430.0, 10.0)));
    let mirror_sphere1 = Entity::DARK(Box::new(Sphere::new(Vec3::new(3.0, 2.0, 7.0), 1.8)), Box::new(GlossyMaterial::new(0.05, Box::new(DiffuseGrayMaterial::new(1.0)))));
    let mirror_sphere2 = Entity::DARK(Box::new(Sphere::new(Vec3::new(0.0, 4.0, 8.0), 2.0)), Box::new(GlossyMaterial::new(0.05, Box::new(DiffuseGrayMaterial::new(1.0)))));
    let glass_sphere_1 = Entity::DARK(Box::new(Sphere::new(Vec3::new(0.6, -4.0, 4.5), 0.9)), Box::new(GlassMaterial));


    let entities = vec![top, bottom, front, back, left, right, sun1, sun3, mirror_sphere1, mirror_sphere2, glass_sphere_1, colored_sphere1];
    let camera = Camera::new(
        Vec3::new(0.0, -9.0, 0.0),
        Quaternion::new(0.0, 10.0, 3.0, 0.0),
        std::f64::consts::PI * 0.35,
        4.0,
        f64::MAX,
        0.01
    );
    Scene::new(entities, camera)
}