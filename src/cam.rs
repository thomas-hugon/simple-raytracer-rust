use crate::point::Point3;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Camera {
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: Point3,
    h_vect: Vec3,
    v_vect: Vec3,
    ll_corner: Point3,
}

impl Camera {
    pub fn new(viewport_height: f64, aspect_ratio: f64, focal_length: f64, origin: Point3) -> Camera {
        let viewport_width = aspect_ratio * viewport_height;
        let h_vect = Vec3(viewport_width, 0., 0.);
        let v_vect = Vec3(0., viewport_height, 0.);
        Camera {
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            h_vect,
            v_vect,
            ll_corner: origin - (h_vect / 2.) - (v_vect / 2.) - Vec3(0., 0., focal_length),
        }
    }

    pub fn ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: Vec3::points(self.origin, self.ll_corner)
                + u * self.h_vect
                + v * self.v_vect,
        }
    }
}
