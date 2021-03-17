use std::sync::Arc;

use crate::material::GenericMaterial;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vec::Vec3;

pub enum Face {
    Front,
    Back,
}

pub struct Intersection {
    pub hit_point: Point3,
    //la normale est toujours stocké de sens opposé par rapport au rayon lancé
    pub normale: Vec3,
    pub face: Face,
    pub factor: f64,
    pub material: GenericMaterial,
}

impl Intersection {
    pub fn new(
        ray: &Ray,
        factor: f64,
        hit_point: Point3,
        outward_normale: Vec3,
        material: GenericMaterial,
    ) -> Intersection {
        // normale: centre -> hitpoint
        // si rayon sens opposé par rapport à normale -> on voit en direction du centre, donc la face ext
        // sinon rayon meme sens: la cam est entre le centre et le hitpoint donc face int
        // meme sens si produit scalaire > 0
        // si la normale.rayon
        if ray.direction.scalar_product(outward_normale) < 0. {
            Intersection {
                hit_point,
                normale: outward_normale,
                face: Face::Front,
                factor,
                material,
            }
        } else {
            Intersection {
                hit_point,
                normale: -outward_normale,
                face: Face::Back,
                factor,
                material,
            }
        }
    }
}

pub enum Geometry {
    Sphere(Sphere),
}

impl Geometry {
    pub fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        match self {
            Geometry::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }
}

pub fn sphere(x: f64, y: f64, z: f64, r: f64, material: GenericMaterial) -> Geometry {
    Geometry::Sphere(Sphere {
        centre: Point3(x, y, z),
        radius: r,
        material,
    })
}

impl Intersect for [Arc<Geometry>] {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let mut closest = t_max;
        let mut ret = None;
        for obj in self.iter() {
            if let Some(hit) = obj.intersect(ray, t_min, closest) {
                closest = hit.factor;
                ret = Some(hit);
            }
        }
        ret
    }
}

pub trait Intersect: Send + 'static + Sync {
    fn intersect(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection>;
}

// TODO sortir le materiau, et les lier dans un Objet(Shape, Material)
pub struct Sphere {
    pub centre: Point3,
    pub radius: f64,
    pub(crate) material: GenericMaterial,
}

impl Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Intersection> {
        let x = Vec3::points(self.centre, ray.origin);
        let a = ray.direction.sqr_len();
        let h = x.scalar_product(ray.direction);
        let h2 = h * h;
        let c = x.sqr_len() - self.radius * self.radius;
        let d = h2 - a * c;
        if d >= 0. {
            //2 racines possibles: (-h - d.sqrt()) / a ou (-h + d.sqrt()) / a
            //on ne veut garder que la plus proche, comprise dans l'interval
            //le plus proche de la cam, c'est celui avec la racine la + petite
            // d étant positif, -h - d.sqrt() < -h + d.sqrt(), donc on teste -h - d.sqrt() en premier
            //normale: va du centre  de la sphere vers le hitpoint
            let root = (-h - d.sqrt()) / a;
            if root >= t_min && root <= t_max {
                return Some(Intersection::new(
                    ray,
                    root,
                    ray.at(root),
                    //division par radius plutot que .unit() -> utilisation d'un bug qui reverse la face du matériau en cas de radius negatif
                    Vec3::points(self.centre, ray.at(root)) / self.radius,
                    self.material.clone(),
                ));
            }
            let root = (-h + d.sqrt()) / a;
            if root >= t_min && root <= t_max {
                return Some(Intersection::new(
                    ray,
                    root,
                    ray.at(root),
                    //division par radius plutot que .unit() -> utilisation d'un bug qui reverse la face du matériau en cas de radius negatif
                    Vec3::points(self.centre, ray.at(root)) / self.radius,
                    self.material.clone(),
                ));
            }
        }
        None
    }
}
