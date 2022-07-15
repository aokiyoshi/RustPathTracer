/* use crate::intersection::Intersection; */
use crate::ray;
use crate::scene::Scene;
use crate::vec;
use image::Rgb;

pub fn get_hdri_pix(img: &image::RgbImage, vec: &vec::Vec3) -> vec::Vec3 {
    let theta = vec.z.asin();
    let a1 = (vec.y / vec.x).atan();
    let width = img.width() as f64 - 1.0;
    let height = img.height() as f64 - 1.0;
    let x = 0.5 * (a1 / std::f64::consts::PI) * width + 0.5 * width;
    let y = 0.5 * height - (theta / std::f64::consts::FRAC_PI_2) * 0.5 * height;
    vec::new(
        img[(x as u32, y as u32)][0] as f64,
        img[(x as u32, y as u32)][1] as f64,
        img[(x as u32, y as u32)][2] as f64,
    )
}

fn _fresnel(f0: vec::Vec3, cos_theta: f64) -> vec::Vec3 {
    return f0 + (vec::new(1.0, 1.0, 1.0) - f0) % (1.0 - cos_theta).powi(5);
}

pub fn get_color(scene: &Scene, ray: &ray::Ray, depth: u8) -> vec::Vec3 {
    // Trace the ray
    let hit = scene.trace(ray);

    //Bounced enought
    if depth >= crate::DEPTH {
        return vec::vec0();
    };

    // Nothing was hit. Return background color
    if hit.is_none() {
        return get_hdri_pix(&scene.hdri, &ray.direction);
    };

    // Unwrap intersection result knowing we never get None as result
    let intersection = hit.unwrap();

    // Generating rays
    let point = intersection.point;
    let n = intersection.norm;
    let v = !intersection.v;
    //// Reflection
    let rray = ray::new(
        point,
        v - n % (n.dot(v) * 2.0) + vec::_rng_vec(0.25 * intersection.material.roughness),
    );

    get_color(scene, &rray, depth + 1) % 0.75
        + intersection.material.albedo % (1.0 / std::f64::consts::PI)
}

pub fn sampler(scene: &Scene, x: f64, y: f64) -> Rgb<u8> {
    let ray = &scene.camera.get_ray(x, y);
    let mut color = get_color(scene, ray, 0);
    for _ in 1..crate::SAMPLES {
        color += get_color(scene, ray, 0);
    }
    color.mul(1. / crate::SAMPLES as f64).torgb() //get average color and convert to Rgb type
}
