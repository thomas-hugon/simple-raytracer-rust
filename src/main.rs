mod color;
mod point;
mod ppm;
mod ray;
mod vec;
mod hit;

use crate::color::Color;
use crate::point::Point3;
use crate::ray::Ray;
use crate::vec::Vec3;
use ppm::Ppm;
use std::fs::File;
use std::io::{BufWriter, Write};
use crate::hit::{Hittable, Sphere};

struct StdOutWriter;
impl Write for StdOutWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        print!("{}", String::from_utf8_lossy(buf));
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

fn ray_color(ray: &Ray, objects: &[Box<dyn Hittable>]) -> Color {
    const WHITE: Color = Color(1.0, 1.0, 1.0);
    const BLUE: Color = Color(0.5, 0.7, 1.0);
    // for obj in objects{
    //     if let Some(hit_point) = obj.hit(ray,0.,2.){
    //         let normale = hit_point.normale;
    //         return 0.5 * (WHITE + Color(normale.0, normale.1, normale.2));
    //     }
    // }
    if let Some(hit_point) = objects.hit(ray,0.,2.){
        let normale = hit_point.normale;
        return 0.5 * (WHITE + Color(normale.0, normale.1, normale.2));
    }

    //gradient de couleur (blanc..bleu) pour le fond
    let unit_direction = ray.direction.unit();
    // 0.5 <= t <= 1. si y monte, t monte
    let t = 0.5 * (unit_direction.y() + 1.);
    // plus y élevé => + bleu
    // plus y bas => plus de blanc
    WHITE * (1.0 - t) + BLUE * t
}


fn main() -> std::io::Result<()> {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1024;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    //Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    const ORIGIN: Point3 = Point3(0f64, 0f64, 0f64);
    const HORIZONTAL: Vec3 = Vec3(VIEWPORT_WIDTH, 0f64, 0f64);
    const VERTICAL: Vec3 = Vec3(0f64, VIEWPORT_HEIGHT, 0f64);
    let lower_left_corner: Point3 =
        ORIGIN - (HORIZONTAL / 2f64) - (VERTICAL / 2f64) - Vec3(0f64, 0f64, FOCAL_LENGTH);

    //Render
    let file = File::create("back.ppm")?;

    let mut ppm = Ppm::new(
        BufWriter::with_capacity((IMAGE_WIDTH * 10) as usize, file),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        65536,
    )?;

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    objects.push(Box::new(Sphere{
        centre: Point3(0.3, 0.3, -1.),
        radius: 0.5
    }));
    objects.push(Box::new(Sphere{
        centre: Point3(-0.6, -0.6, -1.),
        radius: 0.2
    }));

    for (horiz_vec, vert_vec) in iterate_vect(IMAGE_WIDTH, IMAGE_HEIGHT, HORIZONTAL, VERTICAL) {
        let r = Ray {
            origin: ORIGIN,
            direction: Vec3::points(ORIGIN, lower_left_corner) + horiz_vec + vert_vec,
        };

        ppm.next_pixel(ray_color(&r, &objects))?;
    }

    Ok(())
}

fn iterate_vect(
    width: u32,
    height: u32,
    horizontal: Vec3,
    vertical: Vec3,
) -> impl Iterator<Item = (Vec3, Vec3)> {
    (0..height)
        .rev()
        .map(move |j| (j as f64 / (height as f64 - 1.)) * vertical)
        .flat_map(move |vert_vec| {
            (0..width)
                .map(move |i| (i as f64 / (width as f64 - 1.)) * horizontal)
                .map(move |horiz_vec| (horiz_vec, vert_vec))
        })
}
