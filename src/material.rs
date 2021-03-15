use crate::color::Color;
use crate::hit::{Face, Hit, Hittable};
use crate::ray::Ray;
use crate::vec::Vec3;
use std::ops::{Mul, Neg};

pub struct Reflexion {
    pub reflected_ray: Ray,
    pub attenuation: Color,
}
// Un matériau à la propriété de reflechir et d'absorber la lumière
// Les rayons réfléchis et réfractés sont propres à chaque matériau
pub trait Material {
    fn scatter(&self, hit: &Hit, incident_ray: &Ray) -> Option<Reflexion>;
}

pub struct Diffuse(pub Color);

impl Material for Diffuse {
    fn scatter(&self, hit: &Hit, incident_ray: &Ray) -> Option<Reflexion> {
        let target = hit.hit_point + hit.normale + Vec3::random_unit_sphere();
        //rayon lancé de hitpoint en direction de target, en récupérant 50% de la luminosité
        Some(Reflexion {
            attenuation: self.0,
            reflected_ray: Ray {
                origin: hit.hit_point,
                direction: Vec3::points(hit.hit_point, target),
            },
        })
    }
}

pub struct Metal {
    pub color: Color,
    pub fuzziness: f64,
}

impl Material for Metal {
    fn scatter(&self, hit: &Hit, incident_ray: &Ray) -> Option<Reflexion> {
        let reflected = reflect(incident_ray.direction, hit.normale);
        if reflected.scalar_product(hit.normale) > 0. {
            Some(Reflexion {
                reflected_ray: Ray {
                    origin: hit.hit_point,
                    direction: reflected + self.fuzziness * Vec3::random_unit_sphere(),
                },
                attenuation: self.color,
            })
        } else {
            None
        }
        // Some()
    }
}

pub struct Dielectric {
    pub refraction_indice: f64,
}

impl Material for Dielectric {
    fn scatter(&self, hit: &Hit, incident_ray: &Ray) -> Option<Reflexion> {
        // double cos_theta = fmin(dot(-unit_direction, rec.normal), 1.0);
        // double sin_theta = sqrt(1.0 - cos_theta*cos_theta);
        //
        // bool cannot_refract = refraction_ratio * sin_theta > 1.0;
        // vec3 direction;
        //
        // if (cannot_refract)
        // direction = reflect(unit_direction, rec.normal);
        // else
        // direction = refract(unit_direction, rec.normal, refraction_ratio);

        let uv = incident_ray.direction.unit();
        let cos_theta = (-uv).scalar_product(hit.normale).min(1.);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();
        let density_ratio = if let Face::Front = hit.face
        { 1. / self.refraction_indice }
        else { self.refraction_indice };

        //reflection interne totale
        //  si rayon a l'interieur et n > n' ex densité 1.5 et 1. pour l'air
        //  sin theta' = 1.5/1 * sin theta. sachant sin theta' est max 1:
        //  1 > 1.5 * sin theta. donc si inverse ( 1.5/1 *sin theta > 1 ==> faux, pas de solution, pas de refraction )
        let total_intern_refl = density_ratio * sin_theta > 1.;
        let direction = if total_intern_refl || reflectance(cos_theta, density_ratio) > rand::random(){
            reflect(uv, hit.normale)
        } else{
            let r_perp = density_ratio * (uv + cos_theta * hit.normale);
            let r_par = (1. - r_perp.sqr_len()).abs().sqrt().neg().mul(hit.normale);
            r_perp + r_par
        };
        // let refracted = refract(
        //     incident_ray.direction,
        //     hit.normale,
        //     ratio,
        // );
        Some(Reflexion {
            attenuation: Color::new(1., 1., 1.),
            reflected_ray: Ray {
                origin: hit.hit_point,
                direction,
            },
        })
    }
}

fn reflect(incident: Vec3, normale: Vec3) -> Vec3 {
    incident - 2. * incident.scalar_product(normale) * normale
}
fn refract(incident: Vec3, normale: Vec3, density_ratio: f64) -> Vec3 {
    let uv = incident.unit();
    let cos_theta = (-uv).scalar_product(normale).min(1.);
    let r_perp = density_ratio * (uv + cos_theta * normale);
    let r_par = (1. - r_perp.sqr_len()).abs().sqrt().neg().mul(normale);
    r_perp + r_par
}
// static double reflectance(double cosine, double ref_idx) {
// // Use Schlick's approximation for reflectance.
// auto r0 = (1-ref_idx) / (1+ref_idx);
// r0 = r0*r0;
// return r0 + (1-r0)*pow((1 - cosine),5);
// }
fn reflectance(cosinus: f64, ratio: f64) -> f64{
    //schlick approximation
    let r0 = (1. - ratio)/(1. + ratio);
    let r0 = r0 * r0;
    r0 + (1. - r0)*((1. - cosinus).powi(5))
}