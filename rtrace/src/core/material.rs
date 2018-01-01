use core::world::RayCaster;

use core::intersection::RayIntersection;

use core::color::{Color, FresnelIndex};
use defs::DefNumType;

pub trait ColorCalculator {
    fn new() -> Self;
    fn get_color(&self, itersection: &RayIntersection, ray_caster: &RayCaster) -> Option<Color>;
}

pub struct Material {
    ambient: Option<Color>,
    diffuse: Option<Color>,
    specular: Option<(Color, DefNumType)>,
    reflect: Option<FresnelIndex>,
    refract: Option<FresnelIndex>,
}

impl Material {
    pub fn new_useless() -> Self {
        Self { ambient: None,
               diffuse: None,
               specular: None,
               reflect: None,
               refract: None,
        }
    }

    pub fn new_diffuse(diffuse: Color, ambient: Option<Color>) -> Self {
        Self { diffuse: Some(diffuse),
               ambient: ambient,
               specular: None,
               reflect: None,
               refract: None}
    }

    pub fn new_shiny(diffuse: Color, specular: (Color, DefNumType), ambient: Option<Color>) -> Self {
        Self { diffuse: Some(diffuse),
               ambient: ambient,
               specular: Some(specular),
               reflect: None,
               refract: None}
    }

    pub fn new_reflective(reflect: FresnelIndex, diffuse: Option<Color>, specular: Option<(Color, DefNumType)>, ambient: Option<Color>) -> Self {
        Self { diffuse: diffuse,
               ambient: ambient,
               specular: specular,
               reflect: Some(reflect),
               refract: None}
    }

    pub fn new_refractive(refract: FresnelIndex, reflect: Option<FresnelIndex>, diffuse: Option<Color>, specular: Option<(Color, DefNumType)>, ambient: Option<Color>) -> Self {
        Self { diffuse: diffuse,
               ambient: ambient,
               specular: specular,
               reflect: reflect,
               refract: Some(refract)}
    }

    pub fn get_ambient_color(&self) -> Option<&Color> {
        self.ambient.as_ref()
    }

    pub fn get_diffuse_color(&self) -> Option<&Color> {
        self.diffuse.as_ref()
    }

    pub fn get_specular_color(&self) -> Option<&(Color, DefNumType)> {
        self.specular.as_ref()
    }

    pub fn get_fresnel_reflect_index(&self) -> Option<&Color> {
        self.reflect.as_ref()
    }

    pub fn get_fresnel_refract_index(&self) -> Option<&Color> {
        self.refract.as_ref()
    }

    pub fn is_opaque(&self) -> bool {
        self.refract.is_none()
    }

    pub fn is_transparent(&self) -> bool {
        !self.is_opaque()
    }

    pub fn get_transparency_to_light(&self) -> Option<Color> {
        if self.is_transparent() {
            match self.diffuse {
                None => Some(Color::one()),
                Some(color) => Some(Color::one() - color)
            }
        } else {
            None
        }
    }
}