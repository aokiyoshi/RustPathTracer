use crate::vec::Vec3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

pub fn new(origin: Vec3, direction: Vec3) -> Ray {
    Ray {
        origin,
        direction,
    }
}
