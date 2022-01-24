use crate::point::Point3;
use crate::vec::Vec3;
use crate::geometry::{Geometry, Intersect};
use std::sync::Arc;
use crate::color::Color;

//Un rayon lancé est caractérisé par son origine et un vecteur définissant sa direction
pub struct Ray {
    pub color: Color,
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, fact: f64) -> Point3 {
        self.origin + self.direction * fact
    }

    pub fn ray_color(&self, objects: &[Arc<Geometry>], rec_depth: u16) -> Color {
        const BLACK: Color = Color::new(0., 0., 0.);
        //si le rayon a trop rebondi, il n'y a peu de lumière qui peut venir de cette direction -> noir
        if rec_depth == 0 {
            return BLACK;
        }

        // 0.001 pour être sûr d'être > 0. car à cause de l'erreur d'echantillon, lors d'une reflection, le point de deépart peut se
        // trouver legerement avant 0 (-0.000000000000000000001), et donc rebondir sur la surface intérieure de l'objet -> obscurcissement
        // -> http://www.opengl-tutorial.org/intermediate-tutorials/tutorial-16-shadow-mapping/#shadow-acne
        if let Some(hit) = objects.intersect(self, 0.001, f64::INFINITY) {
            // le hit avec le materiau définit si il doit y avoir un rayon reflechi/refracté, et avec quelle attenuation
            // l'attenuation est la couleur de l'objet 0 <= (r,g,b) <= 1
            // un rayon secondaire est lancé depuis le hit point dans la direction du rayon réfléchi/refracté, etc...
            // récursivité: chaque rayon réfl/refr peut frapper un autre objet et rebondir en fonction du matériau
            if let Some(scattered_ray) = hit.material.scatter(&hit, self) {
                // le nombre de rebonds va impacter la luminosité et la couleur
                scattered_ray.color * scattered_ray.ray_color( objects, rec_depth - 1)
            } else {
                //absorption totale si HIT mais pas de rayon réfléchi/réfracté
                BLACK
            }
        } else {
            const WHITE: Color = Color::new(1., 1., 1.);
            const BLUE: Color = Color::new(0.7, 0.85, 1.0);

            let t = 0.5 * (self.direction.unit().y() + 1.);
            WHITE * (1.0 - t) + BLUE * t
        }
    }
}
