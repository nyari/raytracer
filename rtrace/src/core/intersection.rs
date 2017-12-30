use defs::VectorColumn4;
use defs::DefNumType;
use defs::Point3;

use core::Ray;
use core::Model;

use ::na;

#[derive(Debug)]
pub struct RayIntersection<'ray> {
    normal : VectorColumn4,
    point : VectorColumn4,
    itersector_ray : &'ray Ray,
    distance_to_intersection : DefNumType,
}

impl<'ray> RayIntersection<'ray> {
    //Panic if "from_homogeneous" fails
    pub fn new(normal: VectorColumn4, point: VectorColumn4, ray: &'ray Ray) -> Self {
        Self {  normal: normal, 
                point: point, 
                itersector_ray: ray,
                distance_to_intersection: na::distance(&Point3::from_homogeneous(ray.get_origin()).unwrap(),
                                                       &Point3::from_homogeneous(point).unwrap()) 
             }
    }

    pub fn get_intersection_point(&self) -> VectorColumn4 {
        self.point
    }

    pub fn get_normal_vector(&self) -> VectorColumn4 {
        self.normal
    }

    pub fn get_distance_to_intersection(&self) -> DefNumType {
        self.distance_to_intersection
    }

    pub fn get_ray_travel_distance(&self) -> DefNumType {
        self.itersector_ray.get_distance_to_origin()
    }

    pub fn get_ray_depth_counter(&self) -> i32 {
        self.itersector_ray.get_depth_counter()
    }

    pub fn get_ray_inside_counter(&self) -> i32 {
        self.itersector_ray.get_inside_counter()
    }
}


#[cfg(test)]
mod tests {

}