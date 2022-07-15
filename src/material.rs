use crate::vec::Vec3;

#[derive(Clone, Copy)]
pub struct Material {
    pub albedo: Vec3,
    pub roughness: f64,
    pub metalness: f64,
    pub fresnel: f64,
}

pub fn new(albedo: Vec3, roughness: f64, metalness: f64, fresnel: f64) -> Material {
    Material {
        albedo,
        roughness,
        metalness,
        fresnel,
    }
}
