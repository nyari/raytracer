use defs::Matrix4;
use defs::Color;

use core::ray::Ray;
use core::intersection::RayIntersection;
use core::world::World;

pub trait Model {
    fn get_model_view_matrix(&self) -> Option<Matrix4>;
    fn get_intersection<'ray> (&self, ray: &'ray Ray) -> Option<RayIntersection<'ray>>;
    fn get_color(&self, intersection: &RayIntersection, world : &World) -> Option<Color>;
}