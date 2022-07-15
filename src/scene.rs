use crate::camera::Camera;
use crate::intersection::Intersection;
use crate::ray::Ray;
use crate::shader::sampler;
use crate::sphere::Sphere;

use std::sync::{mpsc::channel, Arc, Mutex};
use threadpool::ThreadPool;

use image::{ImageBuffer, Pixel, RgbaImage};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Sphere>,
    pub hdri: image::RgbImage,
}

impl Scene {
    pub fn trace(&self, ray: &Ray) -> Option<Intersection> {
        let mut result = None;
        for obj in &self.objects {
            let temp = obj.intersect(&ray);
            result = match (temp.as_ref(), result.as_ref()) {
                (Some(_), None) => temp,
                (None, _) => result,
                (Some(_), Some(_)) => {
                    match temp.as_ref().unwrap().distance < result.as_ref().unwrap().distance {
                        true => temp,
                        false => result,
                    }
                }
            }
        }
        result
    }

    pub fn calculate(self) -> RgbaImage {
        println!("Job started...");
        let (width, height) = (self.camera.w_screen as u32, self.camera.h_screen as u32);
        let mut img = ImageBuffer::new(width, height);

        let pool = ThreadPool::new(num_cpus::get());
        println!("Num of threads: {}", num_cpus::get());
        let (tx, rx) = channel();

        let guard = Arc::new(Mutex::new(self));

        for y in 0..height {
            let tx = tx.clone();
            let data = Arc::clone(&guard);
            pool.execute(move || {
                for x in 0..width {
                    let scene = data.lock().unwrap();
                    let pixel = sampler(&*scene, x as f64, y as f64);
                    let pixel2 = Pixel::to_rgba(&pixel);
                    tx.send((x, y, pixel2)).expect("Could not send data!");
                }
            });
        }

        for _ in 0..(width * height) {
            let (x, y, pixel) = rx.recv().unwrap();
            img.put_pixel(x, y, pixel);
        }
        img
    }
}
