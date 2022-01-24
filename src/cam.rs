use crate::angle::Angle;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vec::Vec3;
use crate::color::Color;

#[derive(Clone)]
pub struct Camera {
    origin: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    h_vect: Vec3,
    v_vect: Vec3,
    ll_corner: Point3,
    lens_radius: f64
}

impl Camera {
    pub fn new(
        vertical_field_of_view: Angle,
        aspect_ratio: f64,
        aperture: f64,
        cam_origin: Point3,
        target_view: Point3,
        up_vector: Vec3,
    ) -> Camera {

        let w = Vec3::points(target_view, cam_origin);
        let focus_dist = w.len();
        let w_unit = w.unit();
        let u = up_vector.cross_product(w_unit).unit();
        let v = w_unit.cross_product(u);

        let h = (vertical_field_of_view.rad() / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;
        let h_vect = focus_dist*viewport_width * u;
        let v_vect = focus_dist*viewport_height * v;
        Camera {
            origin: cam_origin,
            u,
            v,
            w: w_unit,
            h_vect,
            v_vect,
            ll_corner: cam_origin - (h_vect / 2.) - (v_vect / 2.) - focus_dist* w_unit,
            lens_radius: aperture/2.,
        }
    }

    pub fn ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_unit_sphere();
        let offset = self.u * rd.x() + self.v * rd.y();
        let direction = Vec3::points(self.origin, self.ll_corner) - offset
            + s * self.h_vect
            + t * self.v_vect;
        Ray {
            origin: self.origin + offset,
            direction,
            color: Color::new(0., 0., 0.)
        }
    }
}
