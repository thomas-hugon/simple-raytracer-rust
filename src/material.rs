#![allow(dead_code)]
use crate::color::Color;
use crate::hit::{Face, Hit};
use crate::ray::Ray;
use crate::vec::Vec3;
use std::ops::{Mul, Neg};

pub struct Reflexion {
    pub reflected_ray: Ray,
    pub attenuation: Color,
}
// Un matériau à la propriété de reflechir et d'absorber la lumière
// Les rayons réfléchis ou réfractés sont propres à chaque matériau
//TODO pourquoi pas un struct plutot?
//     chaque materiau peut avoir des caractéristiques de diffusion, de reflexion, de refraction... a voir
pub trait Material {
    //un hit ne renvoi qu'un seul rayon max
    //si il faut reflexion et refraction en même temps, alors chaque sample devra être l'un ou l'autre
    //on peut jouer sur un facteur probabiliste
    fn scatter(&self, hit: &Hit, incident_ray: &Ray) -> Option<Reflexion>;
}

//Un materiau diffus, réémet l'onde lumineuse dans des directions aléatoires
pub struct Diffuse(pub Color);
impl Diffuse{
    pub fn new(r: f64, g: f64, b: f64) -> Diffuse{
        Diffuse(Color::new(r, g, b))
    }
}

impl Material for Diffuse {
    fn scatter(&self, hit: &Hit, _: &Ray) -> Option<Reflexion> {
        //on calcul un vec de diffusion au hasard a partir du hitpoint
        //le point cible pour calcul du vec se trouve dans la sphere unitaire normal exterieur
        //FIXME -> utiliser un point sur la surface de la sphere plutot que dans le cercle => loi de lambert
        let target = hit.hit_point + hit.normale + Vec3::random_unit_sphere();
        Some(Reflexion {
            attenuation: self.0,
            reflected_ray: Ray {
                origin: hit.hit_point,
                direction: Vec3::points(hit.hit_point, target),
            },
        })
    }
}

//Un matériau métallique, réémet l'intégralité le l'onde lumineuse dans un angle theta = -alpha
//alpha étant l'angle entre le rayon d'incidence et la normale
//la réflexion est totale... sauf en cas d'orthogonalité avec la normale
pub struct Metal {
    pub color: Color,
    pub fuzziness: f64,
}
impl Metal{
    pub fn new(r: f64, g: f64, b: f64, fuzziness: f64) -> Metal{
        Metal{
            color: Color::new(r, g, b),
            fuzziness
        }
    }
}

impl Material for Metal {
    // Un metal c'est 100% de reflexion, selon un angle relatif a l'angle d'incidence
    // loi de snell pour la réflexion.
    // on ajoute un facteur de brouillage pour rendre flou la réflexion
    // le facteur de brouillage est une petite sphere de diffusion (loi de lambert) autour du rayon de reflexion
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
    }
}

//Un matériau diélectrique réfracte une grosse partie de la lumière, et en réfléchit une autre dans certaines conditions
//(selon le rapport des densité n1/n2, et l'angle qui peut rendre la reflexion totale)
pub struct Dielectric {
    pub refraction_indice: f64,
}

impl Dielectric{
    pub fn new(refr_indice: f64) -> Dielectric {
        Dielectric{
            refraction_indice: refr_indice,
        }
    }
}

impl Material for Dielectric {
    // Un matériau dielectrique refracte la lumière selon le rapport des indices de refraction des 2 matériaux
    // Une partie est réfléchie, selon un facteur de reflectance de lambert (un seul rayon calculé, donc approche probabiliste)
    // selon l'angle et les densité des matériaux, la lumière peut être piégée: reflexion totale interne
    fn scatter(&self, hit: &Hit, incident_ray: &Ray) -> Option<Reflexion> {
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

// fn refract(incident: Vec3, normale: Vec3, density_ratio: f64) -> Vec3 {
//     let uv = incident.unit();
//     let cos_theta = (-uv).scalar_product(normale).min(1.);
//     let r_perp = density_ratio * (uv + cos_theta * normale);
//     let r_par = (1. - r_perp.sqr_len()).abs().sqrt().neg().mul(normale);
//     r_perp + r_par
// }

fn reflectance(cosinus: f64, ratio: f64) -> f64{
    //schlick approximation
    let r0 = (1. - ratio)/(1. + ratio);
    let r0 = r0 * r0;
    r0 + (1. - r0)*((1. - cosinus).powi(5))
}