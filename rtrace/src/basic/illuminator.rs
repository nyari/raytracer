use core::{RayIntersection, RayCaster, LightIntersection, LightSourceVec, Illuminator};

pub struct SimpleIlluminator {
    lights : LightSourceVec
}

impl Illuminator for SimpleIlluminator {
    fn get_illumination_at(&self, intersection: &RayIntersection, illumination_caster: &RayCaster) -> Vec<LightIntersection> {
        self.lights.iter().filter_map(|light| {
            match light.get_ray_to_intersection(intersection) {
                None => None,
                Some(ray) => {
                    match illumination_caster.cast_light_ray(&ray, intersection) {
                        None => None,
                        Some(illumintaion_shadowing) => {
                            match light.get_illumination_at(intersection) {
                                None => None,
                                Some(illumination) => {
                                    Some(illumination.get_shadowed(&illumintaion_shadowing))
                                }
                            }
                        }
                    }
                }
            }
        }).collect()
    }
}