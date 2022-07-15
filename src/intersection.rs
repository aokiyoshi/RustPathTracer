use crate::material;
use crate::vec::Vec3;

pub struct Intersection {
    pub point: Vec3,
    pub norm: Vec3,
    pub distance: f64,
    pub v: Vec3,
    pub material: material::Material,
}
