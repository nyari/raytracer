use core::color::Color;

use core::ray::Ray;
use core::intersection::RayIntersection;

pub trait LightSource {
    fn get_ray_to_intersection(&self, intersection: &RayIntersection) -> Option<Ray>;
    fn get_illumination_at(&self, intersection: &RayIntersection) -> Option<Color>;
}