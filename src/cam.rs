use crate::point::Point3;
use crate::ray::Ray;
use crate::vec::Vec3;
use crate::angle::Angle;

pub struct Camera {
    origin: Point3,
    h_vect: Vec3,
    v_vect: Vec3,
    ll_corner: Point3,
}

impl Camera {
    pub fn new( vertical_field_of_view: Angle, aspect_ratio: f64, cam_origin: Point3, target_view: Point3, up_vector: Vec3) -> Camera {
        let w = Vec3::points(target_view, cam_origin).unit();
        let u = up_vector.cross_product(w).unit();
        let v = w.cross_product(u);

        let h = (vertical_field_of_view.rad()/2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let h_vect = viewport_width * u;
        let v_vect = viewport_height * v;
        Camera {
            origin: cam_origin,
            h_vect,
            v_vect,
            ll_corner: cam_origin - (h_vect / 2.) - (v_vect / 2.) - w,
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
