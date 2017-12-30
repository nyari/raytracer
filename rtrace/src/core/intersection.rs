use defs::VectorColumn4;
use defs::DefNumType;
use defs::Point3;

use core::Ray;
use core::Model;

use ::na;

pub struct RayIntersection<'ray, 'model> {
    normal : VectorColumn4,
    point : VectorColumn4,
    intersector_ray : &'ray Ray,
    intersected_model : &'model Model,
    distance_to_intersection : DefNumType,
}

impl<'ray, 'model> RayIntersection<'ray, 'model> {
    //Panic if "from_homogeneous" fails
    pub fn new(normal: VectorColumn4, point: VectorColumn4, ray: &'ray Ray, model: &'model Model) -> Self {
        Self {  normal: normal, 
                point: point, 
                intersector_ray: ray,
                intersected_model: model,
                distance_to_intersection: na::distance(&Point3::from_homogeneous(*ray.get_origin()).unwrap(),
                                                       &Point3::from_homogeneous(point).unwrap()) 
             }
    }

    pub fn get_intersection_point(&self) -> &VectorColumn4 {
        &self.point
    }

    pub fn get_normal_vector(&self) -> &VectorColumn4 {
        &self.normal
    }

    pub fn get_distance_to_intersection(&self) -> DefNumType {
        self.distance_to_intersection
    }

    pub fn get_ray_travel_distance(&self) -> DefNumType {
        self.intersector_ray.get_distance_to_origin()
    }

    pub fn get_ray_depth_counter(&self) -> i32 {
        self.intersector_ray.get_depth_counter()
    }

    pub fn get_ray_inside_counter(&self) -> i32 {
        self.intersector_ray.get_inside_counter()
    }

    pub fn get_intersected_model(&self) -> &Model {
        self.intersected_model
    }
}


#[cfg(test)]
mod tests {

}