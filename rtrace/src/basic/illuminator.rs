use core::*;
use core::color::Color;

pub struct SimpleIlluminator {
    lights : LightSourceVec
}

impl Illuminator for SimpleIlluminator {
    fn new(lights: LightSourceVec) -> Self {
        Self {lights : lights}
    }

    fn get_lights(&self) -> &LightSourceVec {
        &self.lights
    }

    fn get_lights_mut(&mut self) -> &mut LightSourceVec{
        &mut self.lights
    }

    fn get_illumination_at(&self, intersection: &RayIntersection, illumination_caster: &IlluminationCaster) -> Option<Color> {
        None
        
    }
}