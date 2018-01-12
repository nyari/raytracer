use core::{LightSource, Ray, LightIntersection, RayIntersection, Color};
use defs::{Vector3, Point3, FloatType};
use na;
use na::{Unit};

pub struct DotLightSource {
    color: Color,
    intensity: FloatType,
    position: Point3,
    attenuation_const: FloatType,
    attenuation_linear: FloatType,
    attenuation_squared: FloatType
}

impl DotLightSource {
    pub fn new_natural(color: Color, intensity: FloatType, position: Point3) -> Self {
        Self {  color: color,
                intensity: intensity,
                position: position,
                attenuation_const: 0.0,
                attenuation_linear: 0.0,
                attenuation_squared: 1.0,
        }
    }

    pub fn new_custom_attenuation(color: Color, intensity: FloatType, position: Point3, constant: Option<FloatType>, linear: Option<FloatType>, squared: Option<FloatType>) -> Self {
        Self {  color: color,
                intensity: intensity,
                position: position,
                attenuation_const: match constant {None => 0.0, Some(value) => value },
                attenuation_linear: match linear {None => 0.0, Some(value) => value },
                attenuation_squared: match squared {None => 0.0, Some(value) => value },
        }
    }

    fn get_attenuation(&self, distance: FloatType) -> FloatType {
        ((self.attenuation_squared * distance).powi(2) + self.attenuation_linear * distance + self.attenuation_const).recip()
    }
}

impl LightSource for DotLightSource {
    fn get_ray_to_intersection(&self, intersection: &RayIntersection) -> Option<Ray> {
        let intersection_point = intersection.get_intersection_point();
        let to_intersection_point_vector = intersection_point - self.position;

        Some(Ray::new_single_shot(self.position, to_intersection_point_vector))
    }

    fn get_illumination_at(&self, intersection: &RayIntersection) -> Option<LightIntersection> {
        let intersection_point = intersection.get_intersection_point();
        let distance = na::distance(intersection_point, &self.position);
        let attenuation = self.get_attenuation(distance);
        let intersection_to_light_vector = self.position - intersection_point;

        let result_color = self.color.mul_scalar(&(attenuation * self.intensity));

        Some(LightIntersection::new(result_color, intersection_to_light_vector))
    }
}


pub struct SpotLightSource {
    dot_light: DotLightSource,
    direction: Unit<Vector3>,
    max_angle_rad: FloatType
}

impl SpotLightSource {
    pub fn new(dot_light: DotLightSource, direction: Vector3, max_angle_radian: FloatType) -> Self {
        Self {  dot_light: dot_light,
                direction: Unit::new_normalize(direction),
                max_angle_rad: max_angle_radian}
    }
}

impl LightSource for SpotLightSource {
    fn get_ray_to_intersection(&self, intersection: &RayIntersection) -> Option<Ray> {
        let dot_light_ray = self.dot_light.get_ray_to_intersection(intersection).expect("DotLightSource Ray to Intersection returned None");
        let angle = na::angle(dot_light_ray.get_direction(), self.direction.as_ref());
        if angle < self.max_angle_rad {
            Some(dot_light_ray)
        } else {
            None
        }
    }

    fn get_illumination_at(&self, intersection: &RayIntersection) -> Option<LightIntersection> {
        self.dot_light.get_illumination_at(intersection)
    }
}