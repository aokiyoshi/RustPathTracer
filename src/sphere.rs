use crate::intersection::Intersection;
use crate::material;
use crate::ray::Ray;
use crate::vec::Vec3;

pub struct Sphere {
    pub loc: Vec3,
    pub radius: f64,
    pub material: material::Material,
}

impl Sphere {
    fn norm(&self, point: Vec3) -> Vec3 {
        (point - self.loc).norm()
    }
    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let l = self.loc - ray.origin;
        let adj = l.dot(ray.direction);
        let d2 = l.dot(l) - (adj * adj);
        let radius2 = self.radius * self.radius;
        if d2 > radius2 {
            return None;
        }
        let thc = (radius2 - d2).sqrt();
        let t0 = adj - thc;
        let t1 = adj + thc;
        if t0 < 0.0 || t1 < 0.0 {
            return None;
        }
        let distance = if t0 < t1 { t0 } else { t1 };
        //return section
        let point = ray.direction.norm() % distance + ray.origin;

        Some(Intersection {
            point,
            norm: self.norm(point),
            distance,
            v: (ray.origin - point).norm(),
            material: self.material,
        })
    }
}

pub fn new(loc: Vec3, rad: f64, material: material::Material) -> Sphere {
    Sphere {
        loc,
        radius: rad,
        material,
    }
}
