use core::world::World;
use core::world::RayCaster;

use core::intersection::RayIntersection;

use defs::Color;

pub trait ColorCalculator {
    fn new() -> Self;
    fn get_color(&self, itersection: &RayIntersection, ray_caster: &RayCaster) -> Option<Color>;
}

