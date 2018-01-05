use core::{Ray, RayIntersection, Color, RayCaster};
use defs::Vector3;

pub struct LightIntersection {
    illumination: Color,
    light_direction: Vector3
}

impl LightIntersection {
    pub fn new(illumination: Color, light_direction: Vector3) -> Self {
        Self {  illumination: illumination, 
                light_direction : light_direction.normalize()}
    }

    pub fn get_shadowed(&self, shadowing: &Color) -> Self {
        Self {  illumination: self.illumination * *shadowing,
                ..*self}
    }

    pub fn get_illumination(&self) -> &Color {
        &self.illumination
    }

    pub fn get_light_direction(&self) -> &Vector3 {
        &self.light_direction
    }
}

pub trait LightSource {
    fn get_ray_to_intersection(&self, intersection: &RayIntersection) -> Option<Ray>;
    fn get_illumination_at(&self, intersection: &RayIntersection) -> Option<LightIntersection>;
}

pub trait Illuminator {
    fn get_illumination_at(&self, intersection: &RayIntersection, illumination_caster: &RayCaster) -> Vec<LightIntersection>;
}