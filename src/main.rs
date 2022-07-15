extern crate piston_window;

mod camera;
mod intersection;
mod material;
mod ray;
mod scene;
mod shader;
mod sphere;
mod vec;

use camera::Camera;
use scene::Scene;
use std::path::Path;

const DEPTH: u8 = 16;
const SAMPLES: usize = 16;
const SCREEN_W: u32 = 256;
const SCREEN_H: u32 = 256;

fn main() {
    // Setting up
    let ratio = SCREEN_H as f64 / SCREEN_W as f64;
    let hdri: image::RgbImage = image::open(Path::new("./hdri.jpg")).unwrap().to_rgb();
    let f = 0.75;
    let a = vec::new(1.0, 0.0, -1.0).norm();
    let cam = Camera {
        loc: vec::new(0.0, 6.0, 6.0),
        direction: a.norm(),
        h_real: f * ratio,
        w_real: f,
        h_screen: SCREEN_H as f64,
        w_screen: SCREEN_W as f64,
    };

    let m = material::new(vec::new(105.0, 148.0, 135.0), 0.01, 0.0, 1.5);
    let sp = sphere::new(vec::new(6.0, 3.0, 0.0), 3.0, m);

    let m2 = material::new(vec::new(0.0, 148.0, 135.0), 0.7, 0.0, 1.5);
    let sp2 = sphere::new(vec::new(6.0, 12.0, 2.0), 2.0, m2);

    let newscene = Scene {
        camera: cam,
        objects: vec![sp, sp2],
        hdri,
    };

    // Get an image
    let frame_buffer = newscene.calculate();

    let mut window: piston_window::PistonWindow =
        piston_window::WindowSettings::new("Pathtracer", [SCREEN_W, SCREEN_H])
            .build()
            .unwrap();
    let tex = piston_window::Texture::from_image(
        &mut window.create_texture_context(),
        &frame_buffer,
        &piston_window::TextureSettings::new(),
    )
    .unwrap();
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _| {
            piston_window::clear([1.0; 4], g);
            piston_window::image(&tex, c.transform, g)
        });
    }
}
