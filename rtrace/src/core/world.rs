use defs::Color;
use defs::VectorColumn4;
use defs::DefNumType;

use core::model::Model;
use core::light::LightSource;
use core::ray::Ray;
use core::intersection::RayIntersection;
use core::material::ColorCalculator;

type ModelBoxed = Box<Model + 'static>;
type LightSourceBoxed = Box<LightSource + 'static>;

type ModelVec = Vec<ModelBoxed>;
type LightSourceVec = Vec<LightSourceBoxed>;



pub trait RayCaster {
    fn cast_ray(&self, ray: &Ray) -> Option<Color>;
    fn cast_ray_first_intersection_distance(&self, ray: &Ray) -> Option<DefNumType>;
}

pub trait IlluminationCaster {
    fn get_illumination_at(&self, coord: &VectorColumn4) -> Option<Color>;
}

pub trait Intersector {
    fn new() -> Self;
    
    fn get_intersections<'ray, 'model>(&self, ray: &'ray Ray) -> Vec<RayIntersection<'ray, 'model>>;
    fn get_nearest_intersection<'ray, 'model>(&self, ray: &'ray Ray) -> Option<RayIntersection<'ray, 'model>>;
    
    fn get_reverse_intersections<'ray, 'model>(&self, ray: &'ray Ray) -> Vec<RayIntersection<'ray, 'model>>;
    fn get_reverse_nearest_intersection<'ray, 'model>(&self, ray: &'ray Ray) -> Option<RayIntersection<'ray, 'model>>;

    fn build(&mut self, models: &ModelVec);
}

pub trait Illuminator {
    fn new() -> Self;

    fn get_illumination_at(&self, coord: &VectorColumn4, illumination_caster: &IlluminationCaster) -> Option<Color>;

    fn build(&mut self, lights: &LightSourceVec);
}

pub struct World<RayIntersector, ColorResolver, LightSourceResolver> {
    models : ModelVec,
    lights : LightSourceVec,
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
        let mut result = Self {models: models,
                               lights: lights,
                               intersector: RayIntersector::new(),
                               color_resolver: ColorResolver::new(),
                               illuminator: LightSourceResolver::new(),
                               depth_limit: ray_depth_limit};

        result.intersector.build(&result.models);
        result.illuminator.build(&result.lights);
        
        result
    }
}

impl<RayIntersector: Intersector,
     ColorResolver: ColorCalculator,
     LightSourceResolver : Illuminator> RayCaster for World<RayIntersector, ColorResolver, LightSourceResolver> {
    fn cast_ray(&self, ray: &Ray) -> Option<Color> {
        if ray.get_depth_counter() <= self.depth_limit {
            match self.intersector.get_nearest_intersection(ray) {
                Some(nearest_intersection) => self.color_resolver.get_color(&nearest_intersection, self),
                None => None,
            }
        } else {
            None
        }
    }

    fn cast_ray_first_intersection_distance(&self, ray: &Ray) -> Option<DefNumType> {
        if ray.get_depth_counter() <= self.depth_limit {
            let nearest_intersection_option = self.intersector.get_nearest_intersection(ray);
            match self.intersector.get_nearest_intersection(ray) {
                Some(nearest_intersection) => Some(nearest_intersection.get_distance_to_intersection()),
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
    fn get_illumination_at(&self, coord: &VectorColumn4) -> Option<Color> {
        self.illuminator.get_illumination_at(coord, self)
    }
}