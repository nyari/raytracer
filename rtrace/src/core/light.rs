use defs::VectorColumn4;
use defs::Color;

use core::ray::Ray;

pub trait LightSource {
    fn get_ray_to_coordinate(&self, coord: &VectorColumn4) -> Ray;
    fn get_illumination_at(&self, coord: &VectorColumn4) -> Color;
}