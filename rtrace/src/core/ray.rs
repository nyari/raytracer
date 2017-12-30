use defs::VectorTup;
use defs::PointTup;
use defs::DefNumType;
use defs::VectorColumn4;

use tools::conv;
use core::RayIntersection;

#[derive(Debug)]
pub struct Ray {
    direction : VectorColumn4,
    origin : VectorColumn4,
    distance_to_origin : DefNumType,
    inside_counter : i32,
    depth_counter : i32
}

impl Ray {
    pub fn new(origin: PointTup, dir: VectorTup) -> Self {
        Self    { direction: conv::vectcolumn4(dir),
                  origin: conv::vectcolumn4(origin),
                  distance_to_origin: 0.0,
                  inside_counter: 0,
                  depth_counter: 0 }
    }

    pub fn continue_ray(intersection: &RayIntersection, direction: VectorColumn4) -> Self {
        Self    { direction: direction,
                  origin: intersection.get_intersection_point(),
                  distance_to_origin: intersection.get_distance_to_intersection() + intersection.get_ray_travel_distance(),
                  inside_counter: intersection.get_ray_inside_counter(),
                  depth_counter: intersection.get_ray_depth_counter() + 1}
    }

    pub fn get_origin(&self) -> VectorColumn4 {
        self.origin
    }

    pub fn get_direction(&self) -> VectorColumn4 {
        self.direction
    }

    pub fn get_distance_to_origin(&self) -> DefNumType {
        self.distance_to_origin
    }

    pub fn get_inside_counter(&self) -> i32 {
        self.inside_counter
    }

    pub fn get_depth_counter(&self) -> i32 {
        self.depth_counter
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_new1_assignment() {
        let test_ray = Ray::new(PointTup::new(0.0, 0.0, 0.0), VectorTup::new(1.0, 0.0, 0.0));

        assert_eq!(test_ray.get_distance_to_origin(), 0.0);
        assert_eq!(test_ray.get_inside_counter(), 0);
        assert_eq!(test_ray.get_depth_counter(), 0);
    }

    fn test_continue_ray() {

    }
}