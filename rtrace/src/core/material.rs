use core::{RayCaster, Color, FresnelIndex, RayIntersection, IlluminationCaster};

use defs::DefNumType;

pub trait ColorCalculator {
    fn get_color(&self, itersection: &RayIntersection, ray_caster: &RayCaster, illumination_caster: &IlluminationCaster) -> Option<Color>;
}

pub struct FresnelData {
    pub n: FresnelIndex,
    pub n_inverse: FresnelIndex,
    pub n_avg: DefNumType,
    pub n_imaginary: FresnelIndex,
    pub n_imaginary_inverse: FresnelIndex,
    pub n_imaginary_avg: DefNumType,
    pub f0: FresnelIndex,
    pub f0_avg: DefNumType,
    pub f0_inverse: FresnelIndex,
    pub f0_inverse_avg: DefNumType
}

impl FresnelData {
    fn new(real: FresnelIndex, imaginary: FresnelIndex) -> Self {
        let mut f0 = ((real - FresnelIndex::one()) * (real - FresnelIndex::one())) + imaginary * imaginary;
        f0 *=  (((real + FresnelIndex::one()) * (real + FresnelIndex::one())) + imaginary * imaginary).recip();
        let mut f0_inverse = ((imaginary - FresnelIndex::one()) * (imaginary - FresnelIndex::one())) + imaginary * imaginary;
        f0_inverse *=  (((imaginary + FresnelIndex::one()) * (imaginary + FresnelIndex::one())) + imaginary * imaginary).recip(); 

        Self {  n: real,
                n_inverse: real.recip(),
                n_avg: real.intensity_avg(),
                n_imaginary: imaginary,
                n_imaginary_inverse: imaginary.recip(),
                n_imaginary_avg: imaginary.intensity_avg(),
                f0: f0,
                f0_avg: f0.intensity_avg(),
                f0_inverse: f0_inverse,
                f0_inverse_avg: f0_inverse.intensity_avg()
        }
    }

    fn get_fresnel_reflect(&self, view_and_normal_angle_cosine: DefNumType) -> Color {
        (Color::one()-self.f0) * Color::one().mul_scalar(&(1.0 - view_and_normal_angle_cosine).powi(5))
    }

    fn get_fresnel_refract(&self, view_and_normal_angle_cosine: DefNumType) -> Color {
        Color::one() - self.get_fresnel_reflect(view_and_normal_angle_cosine)
    }
}


pub struct Material {
    ambient: Option<Color>,
    diffuse: Option<Color>,
    specular: Option<(Color, DefNumType)>,
    fresnel: Option<FresnelData>,
    reflective: bool,
    refractive: bool,
}

impl Material {
    pub fn new_useless() -> Self {
        Self { ambient: None,
               diffuse: None,
               specular: None,
               fresnel: None,
               reflective: false,
               refractive: false
        }
    }

    pub fn new_diffuse(diffuse: Color, ambient: Option<Color>) -> Self {
        Self { diffuse: Some(diffuse),
               ambient: ambient,
               specular: None,
               fresnel: None,
               reflective: false,
               refractive: false}
    }

    pub fn new_shiny(diffuse: Color, specular: (Color, DefNumType), ambient: Option<Color>) -> Self {
        Self { diffuse: Some(diffuse),
               ambient: ambient,
               specular: Some(specular),
               fresnel: None,
               reflective: false,
               refractive: false}
    }

    pub fn new_reflective(fresnel_real: FresnelIndex, fresnel_imagninary: FresnelIndex, diffuse: Option<Color>, specular: Option<(Color, DefNumType)>, ambient: Option<Color>) -> Self {
        Self { diffuse: diffuse,
               ambient: ambient,
               specular: specular,
               fresnel: Some(FresnelData::new(fresnel_real, fresnel_imagninary)),
               reflective: true,
               refractive: false}
    }

    pub fn new_refractive(fresnel_real: FresnelIndex, fresnel_imagninary: FresnelIndex, diffuse: Option<Color>, specular: Option<(Color, DefNumType)>, ambient: Option<Color>) -> Self {
        Self { diffuse: diffuse,
               ambient: ambient,
               specular: specular,
               fresnel: Some(FresnelData::new(fresnel_real, fresnel_imagninary)),
               reflective: false,
               refractive: true}
    }

    pub fn new_reflective_and_refractive(fresnel_real: FresnelIndex, fresnel_imagninary: FresnelIndex, diffuse: Option<Color>, specular: Option<(Color, DefNumType)>, ambient: Option<Color>) -> Self {
        Self { diffuse: diffuse,
               ambient: ambient,
               specular: specular,
               fresnel: Some(FresnelData::new(fresnel_real, fresnel_imagninary)),
               reflective: true,
               refractive: true}
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

    pub fn is_opaque(&self) -> bool {
        !self.refractive
    }

    pub fn is_transparent(&self) -> bool {
        !self.is_opaque()
    }

    pub fn is_reflective(&self) -> bool {
        self.reflective
    }

    pub fn is_refractive(&self) -> bool {
        self.refractive
    }

    pub fn get_fresnel_data(&self) -> Option<&FresnelData> {
        self.fresnel.as_ref()
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