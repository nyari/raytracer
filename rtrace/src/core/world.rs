use core::{Color, Model, LightSource, Ray, RayIntersection, ColorCalculator};
use tools::{Vector3Extensions};

pub type ModelBoxed = Box<Model + 'static>;
pub type LightSourceBoxed = Box<LightSource + 'static>;

pub type ModelVec = Vec<ModelBoxed>;
pub type LightSourceVec = Vec<LightSourceBoxed>;


pub trait RayCaster {
    fn cast_ray(&self, ray: &Ray) -> Option<Color>;
}

pub trait IlluminationCaster {
    fn cast_light_ray(&self, ray: &Ray, intersection: &RayIntersection) -> Option<Color>;
}

pub trait Intersector {
    fn new(models: ModelVec) -> Self;

    fn get_models(&self) -> &ModelVec;
    fn get_models_mut(&mut self) -> &mut ModelVec;
    
    fn get_intersections_reverse_ordered<'ray>(&self, ray: &'ray Ray) -> Vec<RayIntersection<'ray>>;
    fn get_nearest_intersection<'ray>(&self, ray: &'ray Ray) -> Option<RayIntersection<'ray>>;
}

pub trait Illuminator {
    fn new(lights: LightSourceVec) -> Self;

    fn get_lights(&self) -> &LightSourceVec;
    fn get_lights_mut(&mut self) -> &mut LightSourceVec;

    fn get_illumination_at(&self, intersection: &RayIntersection, illumination_caster: &IlluminationCaster) -> Option<Color>;
}

pub struct World<RayIntersector, ColorResolver, LightSourceResolver> {
    intersector : RayIntersector,
    color_resolver : ColorResolver,
    illuminator: LightSourceResolver,
    depth_limit : i32,
}

impl<RayIntersector: Intersector,
     ColorResolver: ColorCalculator,
     LightSourceResolver : Illuminator> 
     World<RayIntersector, ColorResolver, LightSourceResolver> {
    
    pub fn new(models: ModelVec, lights: LightSourceVec, ray_depth_limit: i32) -> Self {
        Self {intersector: RayIntersector::new(models),
              color_resolver: ColorResolver::new(),
              illuminator: LightSourceResolver::new(lights),
              depth_limit: ray_depth_limit}
    }

    pub fn get_intersector(&self) -> &RayIntersector {
        &self.intersector
    }

    pub fn get_illuminator(&self) -> &LightSourceResolver {
        &self.illuminator
    }

    pub fn get_color_resolver(&self) -> &ColorResolver {
        &self.color_resolver
    }
}

impl<RayIntersector: Intersector,
     ColorResolver: ColorCalculator,
     LightSourceResolver : Illuminator> RayCaster for World<RayIntersector, ColorResolver, LightSourceResolver> {
    fn cast_ray(&self, ray: &Ray) -> Option<Color> {
        if ray.get_depth_counter() <= self.depth_limit {
            match self.get_intersector().get_nearest_intersection(ray) {
                Some(nearest_intersection) => self.get_color_resolver().get_color(&nearest_intersection, self),
                None => None,
            }
        } else {
            None
        }
    }
}

impl<RayIntersector: Intersector,
     ColorResolver: ColorCalculator,
     LightSourceResolver : Illuminator> IlluminationCaster for World<RayIntersector, ColorResolver, LightSourceResolver> {
    fn cast_light_ray(&self, ray: &Ray, intersection: &RayIntersection) -> Option<Color> {
        let origin_to_intersection_vector = intersection.get_intersection_point() - ray.get_origin();

        if !origin_to_intersection_vector.same_direction_as(ray.get_direction()) {
            panic!("Ray not in the coorect direction for obstacle search!");
        }
        
        let max_length = origin_to_intersection_vector.length();
        let mut resulting_color = Color::one();

        for intersection in self.get_intersector().get_intersections_reverse_ordered(ray).iter().rev() {
            if intersection.get_distance_to_intersection() > max_length {
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