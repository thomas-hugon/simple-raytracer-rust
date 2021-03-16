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
use crate::hit::{Hittable, Sphere};
use crate::material::{dielectric, diffuse, metal, Dielectric, Diffuse, GenericMaterial, Metal};
use crate::point::Point3;
use crate::ray::Ray;
use ppm::Ppm;

use std::fs::File;
use std::io::{BufWriter, Write};
use std::rc::Rc;
use crate::angle::Angle;
use crate::angle::Angle::Deg;

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

fn ray_color(ray: &Ray, objects: &[Rc<dyn Hittable>], rec_depth: u16) -> Color {
    const WHITE: Color = Color::new(1., 1., 1.);
    const BLACK: Color = Color::new(0., 0., 0.);
    const BLUE: Color = Color::new(0.5, 0.7, 1.0);

    //si le rayon a trop rebondi, il n'y a peu de lumière qui peut venir de cette direction -> noir
    if rec_depth == 0 {
        return BLACK;
    }

    // 0.001 pour être sûr d'être > 0. car à cause de l'erreur d'echantillon, lors d'une reflection, le point de deépart peut se
    // trouver legerement avant 0 (-0.000000000000000000001), et donc rebondir sur la surface intérieure de l'objet -> obscurcissement
    // -> http://www.opengl-tutorial.org/intermediate-tutorials/tutorial-16-shadow-mapping/#shadow-acne
    if let Some(hit) = objects.hit(ray, 0.001, f64::INFINITY) {
        // le hit avec le materiau définit si il doit y avoir un rayon reflechi/refracté, et avec quelle attenuation
        // l'attenuation est la couleur de l'objet 0 <= (r,g,b) <= 1
        // un rayon secondaire est lancé depuis le hit point dans la direction du rayon réfléchi/refracté, etc...
        // récursivité: chaque rayon réfl/refr peut frapper un autre objet et rebondir en fonction du matériau
        if let Some(reflexion) = hit.material.scatter(&hit, ray) {
            // le nombre de rebonds va impacter la luminosité et la couleur
            reflexion.attenuation * ray_color(&reflexion.reflected_ray, objects, rec_depth - 1)
        } else {
            //absorption totale si HIT mais pas de rayon réfléchi/réfracté
            BLACK
        }
    } else {
        //gradient de couleur (blanc..bleu) pour le fond si pas de HIT
        let t = 0.5 * (ray.direction.unit().y() + 1.);
        WHITE * (1.0 - t) + BLUE * t
    }
}

fn main() -> std::io::Result<()> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 1024;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;

    let camera = Camera::new(
        Angle::Deg(90.),
        ASPECT_RATIO,
        1.0,
        Point3(0f64, 0f64, 0f64),
    );

    //Render
    let file = File::create("back.ppm")?;

    let mut ppm = Ppm::new(
        BufWriter::with_capacity((IMAGE_WIDTH * 10) as usize, file),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        255,
    )?;

    let objects = world_v2();

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            //on lance N rayons par pixel dans l'interval (i..i+1, j..j+1) et on moyenne la couleur
            const SAMPLES_PER_PIXEL: u32 = 100;
            let mut color = Color::EMPTY;

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH as f64 - 1.);
                let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT as f64 - 1.);
                let ray = camera.ray(u, v);
                color = color + ray_color(&ray, &objects, 50);
            }
            color = color / SAMPLES_PER_PIXEL as f64;
            //gamma correction color^(1/gamma), gamma=2
            color = color.map_each(|v| v.sqrt());
            ppm.next_pixel(color)?;
        }
    }
    Ok(())
}

fn world_v2() -> Vec<Rc<dyn Hittable>> {
    let mut objects: Vec<Rc<dyn Hittable>> = Vec::new();

    let rayon = Deg(45.).rad().cos();
    let mat_left = diffuse(0., 0., 1.);
    let mat_right = diffuse(1., 0., 0.);

    objects.push(Rc::new(Sphere::new(-rayon, 0., -1., rayon, mat_left)));
    objects.push(Rc::new(Sphere::new(rayon, 0., -1., rayon, mat_right)));

    objects
}

fn world_v1() -> Vec<Rc<dyn Hittable>> {
    let mut objects: Vec<Rc<dyn Hittable>> = Vec::new();
    //Def1
    let ground_mat = Diffuse::new(0.8, 0.8, 0.0);
    let center_mat = Diffuse::new(0.1, 0.2, 0.5);
    let left_mat = Dielectric::new(1.3);
    let right_mat = Metal::new(0.8, 0.6, 0.2, 0.1);

    //Def2
    let ground_mat = GenericMaterial {
        color: Color::new(0.8, 0.8, 0.0),
        diffusion_factor: 1.,
        reflection_factor: None,
        refraction_indice: 1.,
    };
    let center_mat = GenericMaterial {
        color: Color::new(0.1, 0.2, 0.5),
        diffusion_factor: 1.,
        reflection_factor: None,
        refraction_indice: 1.,
    };
    let left_mat = GenericMaterial {
        color: Color::new(1., 1., 1.),
        diffusion_factor: 0.1,
        reflection_factor: Some(0.),
        refraction_indice: 1.3,
    };
    let right_mat = GenericMaterial {
        color: Color::new(0.8, 0.6, 0.2),
        diffusion_factor: 0.1,
        reflection_factor: Some(1.),
        refraction_indice: 1.,
    };

    //Def3
    let ground_mat = diffuse(0.8, 0.8, 0.0);
    let center_mat = diffuse(0.1, 0.2, 0.5);
    let left_mat = dielectric(1.5);
    let right_mat = metal(0.8, 0.6, 0.2, 0.1);

    let ground = Sphere::new(0., -100.5, -1., 100., ground_mat);
    let center = Sphere::new(0., 0., -1., 0.5, center_mat);
    let left = Sphere::new(-1., 0., -1., -0.4, left_mat);
    let right = Sphere::new(1., 0., -1., 0.5, right_mat);
    objects.push(Rc::new(ground));
    objects.push(Rc::new(center));
    objects.push(Rc::new(left));
    objects.push(Rc::new(right));
    objects
}
