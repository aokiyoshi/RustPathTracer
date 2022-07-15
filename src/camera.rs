use crate::ray;
use crate::vec::Vec3;

#[derive(Clone, Copy)]
pub struct Camera {
    pub loc: Vec3,
    pub direction: Vec3,
    pub h_real: f64,
    pub w_real: f64,
    pub h_screen: f64,
    pub w_screen: f64,
}

impl Camera {
    pub fn get_ray(&self, x: f64, y: f64) -> ray::Ray {
        //Returns a new ray for given screen-space coords
        //x, y coords from top left corner
        let _rel_x = (self.w_screen - x - x) / self.w_screen;
        let _rel_y = (self.h_screen - y - y) / self.h_screen;
        let _xcam = self.direction.cross(&Vec3 {
            x: 0_f64,
            y: 0_f64,
            z: -1_f64,
        });
        let _ycam = self.direction.cross(&_xcam);
        let _xkoef = _rel_x * self.w_real;
        let _ykoef = _rel_y * self.h_real;
        let _xreal = _xcam.norm().mul(_xkoef);
        let _yreal = _ycam.norm().mul(_ykoef);
        let direction = _xreal + _yreal + self.direction;
        let origin = self.loc;
        ray::new(origin, direction.norm())
    }
}
