use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use glam::Vec3;
use crate::geometry::util;
use crate::material::material::Material;
use crate::ptrandom;

pub struct GlassMaterial;

fn get_fresnel(vec_in: &Vec3, vec_normal: Vec3, ior: f32) -> f32 {
    let mut cosi = clamp(vec_in.dot(vec_normal));
    let etai: f32;
    let etat: f32;
    if cosi > 0.0 {
        etai = ior;
        etat = 1.0;
    } else {
        etai = 1.0;
        etat = ior;
    }

    let sint = etai / etat * 0.0_f32.max(1.0 - cosi * cosi).sqrt();
    if sint >= 1.0 {
        return 1.0;
    }
    let cost = 0.0_f32.max(1.0 - sint * sint).sqrt();
    cosi = cosi.abs();
    let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
    let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));

    (rs * rs + rp * rp) / 2.0
}

fn get_refraction_index(wavelength: f32) -> f32 {
    let w2 =  wavelength * wavelength * 1.0e-6;
    (1.0
        + 1.737596950 * w2 / (w2 - 0.0131887070)
        + 0.313747346 * w2 / (w2 - 0.0623068142)
        + 1.898781010 * w2 / (w2 - 155.23629000)
    ).sqrt()
}

fn clamp(v: f32) -> f32 {
    return if v < -1.0 {
        -1.0
    } else if v > 1.0 {
        1.0
    } else {
        v
    }
}

fn get_next_ray(incoming: Ray, intersection: Intersection) -> Ray {
    let mut ior = get_refraction_index(incoming.wavelength);
    let fresnel = get_fresnel(&incoming.direction, intersection.normal, ior);
    let path = ptrandom::get_unit();

    let direction = if path < fresnel {
        util::reflect(incoming.direction, intersection.normal)
    } else {
        let mut cosi = -incoming.direction.dot(intersection.normal);
        let mut normal = intersection.normal;
        if cosi > 0.0 {
            ior = 1.0 / ior;
        }
        else {
            normal = normal * -1.0;
            cosi = -cosi;
        }
        let sin_tsqr = ior * ior * (1.0 - cosi * cosi);
        if sin_tsqr > 1.0 {
            util::reflect(incoming.direction, normal)
        } else {
            incoming.direction * ior + normal * (ior * cosi - (1.0 - sin_tsqr).sqrt())
        }
    };
    Ray::new(intersection.position, direction, incoming.wavelength, incoming.strength)
}

impl GlassMaterial {

}

impl Material for GlassMaterial {
    fn get_next_ray(&self, incoming: Ray, intersection: Intersection) -> Ray {
        super::glass::get_next_ray(incoming, intersection)
    }
}

pub struct GaussianColoredGlassMaterial {
    wavelength: f32,
    deviation: f32
}

impl GaussianColoredGlassMaterial {
    pub fn new(wavelength: f32, deviation: f32) -> GaussianColoredGlassMaterial {
        GaussianColoredGlassMaterial { wavelength, deviation }
    }
}

impl Material for GaussianColoredGlassMaterial {
    fn get_next_ray(&self, incoming: Ray, intersection: Intersection) -> Ray {
        let p = (self.wavelength - incoming.wavelength) / self.deviation;
        let q = (-0.5 * p * p).exp();
        let mut ray = super::glass::get_next_ray(incoming, intersection);
        ray.strength = ray.strength * q;
        ray
    }
}

pub struct BandPassColoredGlassMaterial {
    min_wavelength: f32,
    max_wavelength: f32
}

impl BandPassColoredGlassMaterial {
    pub fn new(min_wavelength: f32, max_wavelength: f32) -> BandPassColoredGlassMaterial {
        BandPassColoredGlassMaterial { min_wavelength, max_wavelength }
    }
}

impl Material for BandPassColoredGlassMaterial {
    fn get_next_ray(&self, incoming: Ray, intersection: Intersection) -> Ray {
        let strength = if incoming.wavelength >= self.min_wavelength && incoming.wavelength < self.max_wavelength {
            incoming.strength
        } else {
            0.0
        };
        let mut ray = super::glass::get_next_ray(incoming, intersection);
        ray.strength = strength;
        ray
    }
}