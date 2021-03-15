mod angle;
mod cam;
mod color;
mod hit;
mod material;
mod point;
mod ppm;
mod ray;
mod vec;

use crate::cam::Camera;
use crate::color::Color;
use crate::hit::{Face, Hittable, Sphere};
use crate::material::{Dielectric, Diffuse, Metal};
use crate::point::Point3;
use crate::ray::Ray;
use crate::vec::Vec3;
use ppm::Ppm;
use rand::Rng;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::ops::Add;
use std::rc::Rc;

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

fn ray_color(ray: &Ray, objects: &[Box<dyn Hittable>], rec_depth: u16) -> Color {
    const WHITE: Color = Color::new(1., 1., 1.);
    const BLACK: Color = Color::new(0., 0., 0.);
    const BLUE: Color = Color::new(0.5, 0.7, 1.0);

    if rec_depth == 0 {
        return BLACK;
    }

    // 0.001 pour être sûr d'être > 0. car à cause de l'erreur d'echantillon, lors d'une reflection, le point de deépart peut se
    // trouver legerement avant 0 (-0.000000000000000000001), et donc rebondir sur la surface intérieure de l'objet -> obscurcissement
    // -> http://www.opengl-tutorial.org/intermediate-tutorials/tutorial-16-shadow-mapping/#shadow-acne
    if let Some(hit) = objects.hit(ray, 0.001, f64::INFINITY) {
        // la 'lumière' est liée à la la reflection diffuse de l'environnement
        // => on emet un rayon depuis le hitpoint dans dans une direction aleatoire comprise dans la sphere unitaire normale (face ext)
        // et on récupère un pourcentage de la lumière émise par ce rayon.
        // -> plusieurs reflexions possibles : recursivité, et au bout d'un nombre MAX on retourne la couleur noire
        // target: point aléatoire dans cette sphere normale unitaire qui va donner le vec de hit->target pour la reflexion
        // if let Face::Back = hit.face {
        //     println!("BACK -----------------------")
        // }
        // else {
        //     println!("----------------------- FRONT")
        // }

        /*let target = hit.hit_point + hit.normale + Vec3::random_unit_sphere();
        //rayon lancé de hitpoint en direction de target, en récupérant 50% de la luminosité
        0.5 * ray_color(
            &Ray {
                origin: hit.hit_point,
                direction: Vec3::points(hit.hit_point, target),
            },
            objects,
            rec_depth - 1,
        )*/

        if let Some(reflexion) = hit.material.scatter(&hit, ray) {
            reflexion.attenuation * ray_color(&reflexion.reflected_ray, objects, rec_depth - 1)
        } else {
            BLACK
        }

        // ray scattered;
        // color attenuation;
        // if (rec.mat_ptr->scatter(r, rec, attenuation, scattered))
        // return attenuation * ray_color(scattered, world, depth-1);
        // return color(0,0,0);
    } else {
        //gradient de couleur (blanc..bleu) pour le fond sir pas de HIT
        let t = 0.5 * (ray.direction.unit().y() + 1.);
        WHITE * (1.0 - t) + BLUE * t
    }
}

fn main() -> std::io::Result<()> {
    //Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 500;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    const SAMPLES_PER_PIXEL: u32 = 50;
    let camera = Camera::new(2.0, ASPECT_RATIO, 1.0, Point3(0f64, 0f64, 0f64));

    //Render
    let file = File::create("back.ppm")?;

    let mut ppm = Ppm::new(
        BufWriter::with_capacity((IMAGE_WIDTH * 10) as usize, file),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        255,
    )?;

    // auto material_ground = make_shared<lambertian>(color(0.8, 0.8, 0.0));
    // auto material_center = make_shared<lambertian>(color(0.7, 0.3, 0.3));
    // auto material_left   = make_shared<metal>(color(0.8, 0.8, 0.8));
    // auto material_right  = make_shared<metal>(color(0.8, 0.6, 0.2));
    //
    // world.add(make_shared<sphere>(point3( 0.0, -100.5, -1.0), 100.0, material_ground));
    // world.add(make_shared<sphere>(point3( 0.0,    0.0, -1.0),   0.5, material_center));
    // world.add(make_shared<sphere>(point3(-1.0,    0.0, -1.0),   0.5, material_left));
    // world.add(make_shared<sphere>(point3( 1.0,    0.0, -1.0),   0.5, material_right));

    let mut objects: Vec<Box<dyn Hittable>> = Vec::new();
    let ground = Sphere {
        centre: Point3(0., -100.5, -1.),
        radius: 100.,
        material: Rc::new(Box::new(Diffuse(Color::new(0.8, 0.8, 0.0)))),
    };
    let center = Sphere {
        centre: Point3(0., 0., -1.),
        radius: 0.5,
        material: Rc::new(Box::new(Diffuse(Color::new(0.1, 0.2, 0.5)))),
    };
    let left = Sphere {
        centre: Point3(-1., 0., -1.),
        radius: -0.4,
        material: Rc::new(Box::new(Dielectric {
            refraction_indice: 1.2,
        })),
    };
    let right = Sphere {
        centre: Point3(1., 0., -1.),
        radius: 0.5,
        material: Rc::new(Box::new(Metal {
            color: Color::new(0.8, 0.6, 0.2),
            fuzziness: 1.,
        })),
    };
    objects.push(Box::new(ground));
    objects.push(Box::new(center));
    objects.push(Box::new(left));
    objects.push(Box::new(right));

    let scale = 1. / (SAMPLES_PER_PIXEL as f64);
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            //on lance N samples par pixel dans l'interval (i..i+1, j..j+1) et on average la couleur
            let mut color = Color::EMPTY;
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH as f64 - 1.);
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT as f64 - 1.);
                let ray = camera.ray(u, v);
                color = color + ray_color(&ray, &objects, 30);
            }
            color = color / SAMPLES_PER_PIXEL as f64;
            //gamma correction color^(1/gamma), gamma=2
            color = color.map_each(|v| v.sqrt());
            ppm.next_pixel(color)?;
        }
    }

    // for (u, v) in (0..IMAGE_HEIGHT).rev().flat_map(|j| {
    //     (0..IMAGE_WIDTH).map(move |i| {
    //         (
    //             (i as f64 / (IMAGE_WIDTH as f64 - 1.)),
    //             (j as f64 / (IMAGE_HEIGHT as f64 - 1.)),
    //         )
    //     })
    // }) {
    //     ppm.next_pixel(ray_color(&camera.ray(u, v), &objects))?;
    // }
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
