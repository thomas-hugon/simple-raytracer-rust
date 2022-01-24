#![allow(dead_code)]

use std::ops::{Mul, Neg};

use crate::color::Color;
use crate::geometry::{Face, Intersection};
use crate::ray::Ray;
use crate::vec::Vec3;

fn reflect(incident: Vec3, normale: Vec3) -> Vec3 {
    incident - 2. * incident.scalar_product(normale) * normale
}

fn reflectance(cosinus: f64, ratio: f64) -> f64 {
    //schlick approximation
    let r0 = (1. - ratio) / (1. + ratio);
    let r0 = r0 * r0;
    r0 + (1. - r0) * ((1. - cosinus).powi(5))
}

#[derive(Clone)]
pub struct GenericMaterial {
    pub color: Color,
    pub diffusion_factor: f64,
    pub reflection_factor: Option<f64>,
    pub refraction_indice: f64,
}

pub fn diffuse(r: f64, g: f64, b: f64) -> GenericMaterial {
    GenericMaterial {
        color: Color::new(r, g, b),
        reflection_factor: None,
        diffusion_factor: 1.,
        refraction_indice: 1.,
    }
}

pub fn metal(r: f64, g: f64, b: f64, fuzziness: f64) -> GenericMaterial {
    GenericMaterial {
        color: Color::new(r, g, b),
        reflection_factor: Some(1.),
        diffusion_factor: fuzziness,
        refraction_indice: 1.,
    }
}

pub fn dielectric(refraction_indice: f64) -> GenericMaterial {
    GenericMaterial {
        color: Color::new(1., 1., 1.),
        reflection_factor: Some(-1.),
        diffusion_factor: 0.,
        refraction_indice,
    }
}
pub fn colored_dielectric(r: f64, g: f64, b: f64, refraction_indice: f64) -> GenericMaterial {
    GenericMaterial {
        color: Color::new(r, g, b),
        reflection_factor: Some(-1.),
        diffusion_factor: 0.,
        refraction_indice,
    }
}

impl GenericMaterial {
    pub(crate) fn scatter(&self, hit: &Intersection, incident_ray: &Ray) -> Option<Ray> {
        //on détermine si reflexion ou refraction
        let mut direction = None;
        //si pas de facteur de reflection, alors -> diffusion totale: reflexion totale dans un rayon diffus autour de la normale
        if self.reflection_factor.is_none() {
            direction = Some(hit.normale);
        }
        let uv = incident_ray.direction.unit();
        //si pas diffusion totale, on détermine aléatoirement si le rayon peut être refracté par rapport au facteur de reflexion
        //donc avec un facteur de reflexion <0 on se retrouve avec une refraction totale (ou quasi selon d'autres facteurs physiques)
        let refraction = if direction.is_none() {
            self.reflection_factor.map_or(false, |reflection_factor| {
                reflection_factor < rand::random()
            })
        } else {
            false
        };

        if refraction {
            let cos_theta = (-uv).scalar_product(hit.normale).min(1.);
            let sin_theta = (1. - cos_theta * cos_theta).sqrt();
            let density_ratio = if let Face::Front = hit.face {
                1. / self.refraction_indice
            } else {
                self.refraction_indice
            };

            //  si rayon a l'interieur et n > n' ex densité 1.5 et 1. pour l'air
            //  sin theta' = 1.5/1 * sin theta. sachant sin theta' est max 1:
            //  1 > 1.5 * sin theta. donc si inverse ( 1.5/1 *sin theta > 1 ==> faux, pas de solution, pas de refraction )
            // on calcul un rayon refracté si non reflection interne totale et non reflectance
            if density_ratio * sin_theta <= 1.
                && reflectance(cos_theta, density_ratio) <= rand::random()
            {
                let r_perp = density_ratio * (uv + cos_theta * hit.normale);
                let r_par = (1. - r_perp.sqr_len()).abs().sqrt().neg().mul(hit.normale);
                direction = Some(r_perp + r_par);
            }
        }
        //si pas de refraction ni de diffusion, c'est un rayon réfléchi
        if direction.is_none() {
            direction = Some(reflect(uv, hit.normale));
        }

        direction
            //si vecteur orthogonaux, pas de rayon ré-émis
            .filter(|direction| direction.scalar_product(hit.normale).abs() > 0.00000000001)
            .map(|direction| {
                //on ajoute un facteur de diffusion
                let direction = if self.diffusion_factor <= 0.00000000001 {
                    direction
                } else {
                    //direction est rayon reflechi ou refracté ou la normale (en cas de diffusion)
                    // self.diffusion_factor * Vec3::random_unit_sphere() => vecteur dans une sphere de rayon self.diffusion_factor, qui part de son centre
                    direction + self.diffusion_factor * Vec3::random_unit_sphere()
                };

                Ray {
                    color: self.color,
                    origin: hit.hit_point,
                    direction,
                }
            })
    }
}
