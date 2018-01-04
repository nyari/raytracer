use core::{Ray, RayIntersection, Color, RayCaster};

pub trait LightSource {
    fn get_ray_to_intersection(&self, intersection: &RayIntersection) -> Option<Ray>;
    fn get_illumination_at(&self, intersection: &RayIntersection) -> Option<Color>;
}

pub trait Illuminator {
    fn get_illumination_at(&self, intersection: &RayIntersection, illumination_caster: &RayCaster) -> Option<Color>;
}