use crate::geometry::intersection::Intersection;
use crate::geometry::ray::Ray;
use crate::geometry::vec3::Vec3;
use crate::material::material::Material;
use crate::ptrandom;

pub struct GlassMaterial;

impl GlassMaterial {

    fn get_fresnel(vec_in: &Vec3, vec_normal: &Vec3, ior: f64) -> f64 {
        let mut cosi = GlassMaterial::clamp(vec_in.dot(vec_normal));
        let etai: f64;
        let etat: f64;
        if cosi > 0.0 {
            etai = ior;
            etat = 1.0;
        } else {
            etai = 1.0;
            etat = ior;
        }

        let sint = etai / etat * 0.0_f64.max(1.0 - cosi * cosi).sqrt();
        if sint >= 1.0 {
            return 1.0;
        }
        let cost = 0.0_f64.max(1.0 - sint * sint).sqrt();
        cosi = cosi.abs();
        let rs = ((etat * cosi) - (etai * cost)) / ((etat * cosi) + (etai * cost));
        let rp = ((etai * cosi) - (etat * cost)) / ((etai * cosi) + (etat * cost));

        (rs * rs + rp * rp) / 2.0
    }

    fn get_refraction_index(wavelength: f64) -> f64 {
        let w2 =  wavelength * wavelength * 1.0e-6;
        (1.0
            + 1.737596950 * w2 / (w2 - 0.0131887070)
            + 0.313747346 * w2 / (w2 - 0.0623068142)
            + 1.898781010 * w2 / (w2 - 155.23629000)
        ).sqrt()
    }

    fn clamp(v: f64) -> f64 {
        return if v < -1.0 {
            -1.0
        } else if v > 1.0 {
            1.0
        } else {
            v
        }
    }
}

impl Material for GlassMaterial {
    fn get_next_ray(&self, incoming: Ray, intersection: Intersection) -> Ray {
        let mut cosi = -incoming.direction.dot(&intersection.normal);
        let mut ior = GlassMaterial::get_refraction_index(incoming.wavelength);
        let fresnel = GlassMaterial::get_fresnel(&incoming.direction, &intersection.normal, ior);
        let path = ptrandom::get_unit();

        let direction = if path < fresnel {
            incoming.direction.reflect(&intersection.normal)
        } else {
            let mut normal = intersection.normal;
            if cosi > 0.0 {
                ior = 1.0 / ior;
            }
            else {
                normal = normal.scale(-1.0);
                cosi = -cosi;
            }
            let sin_tsqr = ior * ior * (1.0 - cosi * cosi);
            if sin_tsqr > 1.0 {
                incoming.direction.reflect(&normal)
            } else {
                incoming.direction.scale(ior).add(&normal.scale(ior * cosi - (1.0 - sin_tsqr).sqrt()))
            }
        };
        Ray::new(intersection.position, direction, incoming.wavelength, incoming.strength)
    }
}