use core::{Color, Ray, RayIntersection, ColorCalculator, Illuminator, Intersector, LightIntersection};
use tools::{Vector3Extensions, CompareWithTolerance};


pub trait RayCaster {
    fn cast_ray(&self, ray: &Ray) -> Option<Color>;
    fn cast_light_ray(&self, ray: &Ray, intersection: &RayIntersection) -> Option<Color>;
}

pub trait IlluminationCaster {
    fn get_illumination_at(&self, intersection: &RayIntersection) -> Vec<LightIntersection>;
}

pub struct World<IntersectorType, ColorCalculatorType, IlluminatorType> {
    intersector : IntersectorType,
    color_calculator : ColorCalculatorType,
    illuminator: IlluminatorType,
    depth_limit : i32,
}

impl<IntersectorType: Intersector + Send + Sync,
     ColorCalculatorType: ColorCalculator + Send + Sync,
     IlluminatorType : Illuminator + Send + Sync> 
     World<IntersectorType, ColorCalculatorType, IlluminatorType> {
    
    pub fn new(intersector: IntersectorType, colorcalc: ColorCalculatorType, illuminator: IlluminatorType, ray_depth_limit: i32) -> Self {
        Self {intersector: intersector,
              color_calculator: colorcalc,
              illuminator: illuminator,
              depth_limit: ray_depth_limit}
    }

    pub fn get_intersector(&self) -> &IntersectorType {
        &self.intersector
    }

    pub fn get_illuminator(&self) -> &IlluminatorType {
        &self.illuminator
    }

    pub fn get_color_calculator(&self) -> &ColorCalculatorType {
        &self.color_calculator
    }
}

impl<IntersectorType: Intersector + Send + Sync,
     ColorCalculatorType: ColorCalculator + Send + Sync,
     IlluminatorType : Illuminator + Send + Sync> RayCaster for World<IntersectorType, ColorCalculatorType, IlluminatorType> {
    fn cast_ray(&self, ray: &Ray) -> Option<Color> {
        if ray.get_depth_counter() <= self.depth_limit {
            match self.get_intersector().get_nearest_intersection(ray) {
                Some(nearest_intersection) => self.get_color_calculator().get_color(&nearest_intersection, self, self),
                None => None,
            }
        } else {
            None
        }
    }

    fn cast_light_ray(&self, ray: &Ray, intersection: &RayIntersection) -> Option<Color> {
        let origin_to_intersection_vector = intersection.get_intersection_point() - ray.get_origin();

        // if !origin_to_intersection_vector.same_direction_as(ray.get_direction()) {
        //     panic!("Ray not in the coorect direction for obstacle search!");
        // }
        
        let max_length = origin_to_intersection_vector.length();
        let mut resulting_color = Color::one();

        for intersection in self.get_intersector().get_intersections_reverse_ordered(ray).iter().rev() {
            if intersection.get_distance_to_intersection().greater_eq_eps(&max_length) {
                return Some(resulting_color)
            }
            
            match intersection.get_material().get_transparency_to_light() {
                None => return None,
                Some(transparency) => resulting_color *= transparency
            }
        }

        Some(resulting_color)
    }
}

impl<IntersectorType: Intersector + Send + Sync,
     ColorCalculatorType: ColorCalculator + Send + Sync,
     IlluminatorType : Illuminator + Send + Sync> IlluminationCaster for World<IntersectorType, ColorCalculatorType, IlluminatorType> {
    fn get_illumination_at(&self, intersection: &RayIntersection) -> Vec<LightIntersection> {
        self.get_illuminator().get_illumination_at(intersection, self)
    }
}