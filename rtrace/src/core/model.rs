use defs::Matrix4;
use defs::Color;

use core::ray::Ray;
use core::intersection::RayIntersection;
use core::world::World;

pub trait Model {
    fn get_model_view_matrix(&self) -> Option<Matrix4>;
    fn get_intersection<'ray, 'model> (&self, ray: &'ray Ray) -> Option<RayIntersection<'ray, 'model>>;
    fn get_ambient_color(&self, intersection: &RayIntersection) -> Option<Color>;
    fn get_duffuse_color(&self, intersection: &RayIntersection) -> Option<Color>;
    fn get_specular_color(&self, intersection: &RayIntersection) -> Option<Color>;
    fn get_fresnel_reflect_color(&self, intersection: &RayIntersection) -> Option<Color>;
    fn get_fresnel_refract_color(&self, intersection: &RayIntersection) -> Option<Color>;
}
