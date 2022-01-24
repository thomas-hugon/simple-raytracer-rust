use std::fs::File;
use std::io::{BufWriter, Write};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

use rand::Rng;

use ppm::Ppm;

use crate::angle::Angle;
use crate::cam::Camera;
use crate::color::Color;
use crate::geometry::{sphere, Geometry};
use crate::material::{colored_dielectric, dielectric, diffuse, metal, GenericMaterial};
use crate::point::Point3;
use crate::vec::Vec3;

mod angle;
mod cam;
mod color;
mod geometry;
mod material;
mod point;
mod ppm;
mod ray;
mod vec;

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

fn main() -> std::io::Result<()> {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = 600;
    const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    const SAMPLES_PER_PIXEL: u32 = 100;
    const MAX_REFLECTION: u16 = 50;

    let camera = Camera::new(
        Angle::Deg(20.),
        ASPECT_RATIO,
        0.07,
        Point3(13., 2., 3.),
        Point3(0., 0., 0.),
        Vec3(0., 1., 0.),
    );

    let file = File::create("back.ppm")?;

    let mut ppm = Ppm::new(
        BufWriter::with_capacity((IMAGE_WIDTH * 13) as usize, file),
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        255,
    )?;

    let objects = world_v4();

    struct WorkFinished(u32, Vec<Color>);
    let (worker_tx, main_rx) = std::sync::mpsc::channel();
    let lines_count = Arc::new(AtomicU32::new(0));
    let workers: Vec<_> = (0..6)
        .map(|_| {
            let lines_count = Arc::clone(&lines_count);
            let camera = camera.clone();
            let objects = objects.clone();
            let worker_tx = worker_tx.clone();
            std::thread::spawn(move || {
                let mut current_line = lines_count.fetch_add(1, Ordering::SeqCst);
                while current_line < IMAGE_HEIGHT {
                    let j = IMAGE_HEIGHT - current_line;
                    let mut colors = Vec::with_capacity(IMAGE_WIDTH as usize);
                    for i in 0..IMAGE_WIDTH {
                        let mut color = Color::EMPTY;

                        for _ in 0..SAMPLES_PER_PIXEL {
                            let u = (i as f64 + rand::random::<f64>()) / (IMAGE_WIDTH as f64 - 1.);
                            let v = (j as f64 + rand::random::<f64>()) / (IMAGE_HEIGHT as f64 - 1.);
                            let ray = camera.ray(u, v);
                            color = color + ray.ray_color(&objects, MAX_REFLECTION);
                        }
                        color = color / SAMPLES_PER_PIXEL as f64;
                        //gamma correction color^(1/gamma), gamma=2
                        color = color.map_each(|v| v.sqrt());
                        colors.push(color);
                    }
                    worker_tx.send(WorkFinished(current_line, colors)).unwrap();

                    current_line = lines_count.fetch_add(1, Ordering::SeqCst);
                }
            })
        })
        .collect();

    let mut lines: Vec<(u32, Vec<Color>)> = Vec::new();
    let mut x = 0;
    for _ in 0..IMAGE_HEIGHT {
        let WorkFinished(j2, colors) = main_rx.recv().unwrap();
        // assert_eq!(j, j2);
        if x == j2 {
            println!("{} lines remaining", IMAGE_HEIGHT - x);
            ppm.next_pixels(&colors)?;
            x += 1;
        } else {
            lines.push((j2, colors));
            lines.sort_by_key(|a: &(u32, Vec<_>)| IMAGE_HEIGHT - a.0);
            while let Some((j3, colors)) = lines.pop() {
                if j3 == x {
                    println!("{} lines remaining", IMAGE_HEIGHT - x);
                    ppm.next_pixels(&colors)?;
                    x += 1;
                } else {
                    lines.push((j3, colors));
                    break;
                }
            }
        }
    }
    lines.sort_by_key(|(l, _)| *l);
    for (j, colors) in lines {
        println!("{} lines remaining", IMAGE_HEIGHT - j);
        ppm.next_pixels(&colors)?;
    }
    for worker in workers {
        worker.join().unwrap();
    }
    Ok(())
}

fn world_v5() -> Vec<Arc<Geometry>> {
    let mut objects: Vec<Arc<Geometry>> = Vec::new();

    let ground_material = diffuse(0.5, 0.5, 0.5);
    objects.push(Arc::new(sphere(0., -1000., 0., 1000., ground_material)));

    let material2 = diffuse(0.4, 0.2, 0.1);
    objects.push(Arc::new(sphere(-4., 1., 0., 1.0, material2)));

    let bubble = GenericMaterial {
        color: Color::new(1., 0.9, 0.9),
        reflection_factor: Some(0.02),
        diffusion_factor: 0.,
        refraction_indice: 0.99,
    };
    objects.push(Arc::new(sphere(0., 1., 0., 1.0, bubble)));

    let material3 = metal(0.7, 0.6, 0.5, 0.0);
    objects.push(Arc::new(sphere(4., 1., 0., 1.0, material3)));

    let material2 = diffuse(0.2, 0.6, 0.1);
    objects.push(Arc::new(sphere(-4., 1., -4., 1.0, material2)));

    objects
}

fn world_v4() -> Vec<Arc<Geometry>> {
    let mut objects: Vec<Arc<Geometry>> = Vec::new();

    let ground_material = diffuse(0.5, 0.5, 0.5);
    objects.push(Arc::new(sphere(0., -1000., 0., 1000., ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = Point3(
                a as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::random::<f64>(),
            );

            if Vec3::points(Point3(4., 0.2, 0.), center).len() > 0.9 {
                let sphere_material;

                if choose_mat < 0.4 {
                    // diffuse
                    let Color { red, green, blue } = Color::random() * Color::random();
                    sphere_material = diffuse(red, green, blue);
                    objects.push(Arc::new(sphere(
                        center.0,
                        center.1,
                        center.2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.65 {
                    // metal
                    let Color { red, green, blue } = Color::random_range(0.5..1.);
                    let fuzz = rand::thread_rng().gen_range(0.0..0.5);
                    sphere_material = metal(red, green, blue, fuzz);
                    objects.push(Arc::new(sphere(
                        center.0,
                        center.1,
                        center.2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.85 {
                    // glass
                    let Color { red, green, blue } =
                        Color::random().map_each(|v| v.sqrt().sqrt().sqrt().sqrt());
                    sphere_material = colored_dielectric(red, green, blue, 1.5);
                    objects.push(Arc::new(sphere(
                        center.0,
                        center.1,
                        center.2,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // bubble
                    let Color { red, green, blue } =
                        Color::random().map_each(|v| v.sqrt().sqrt().sqrt().sqrt());
                    let bubble = GenericMaterial {
                        color: Color::new(red, green, blue),
                        reflection_factor: Some(0.02),
                        diffusion_factor: 0.,
                        refraction_indice: 0.99,
                    };
                    objects.push(Arc::new(sphere(
                        center.0,
                        center.1 + 0.3 + 1.8 * rand::random::<f64>(),
                        center.2 - (0.15 * rand::random::<f64>()),
                        0.2,
                        bubble,
                    )));
                }
            }
        }
    }

    let material1 = dielectric(1.5);
    objects.push(Arc::new(sphere(0., 1., 0., 1.0, material1)));

    let material2 = diffuse(0.4, 0.2, 0.1);
    objects.push(Arc::new(sphere(-4., 1., 0., 1.0, material2)));

    let material3 = metal(0.7, 0.6, 0.5, 0.0);
    objects.push(Arc::new(sphere(4., 1., 0., 1.0, material3)));

    objects
}
