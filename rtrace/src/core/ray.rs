use defs::Vector3;
use defs::Point3;
use defs::DefNumType;
use defs::VectorColumn4;

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
    pub fn new(origin: Point3, dir: Vector3) -> Self {
        Self    { direction: dir.to_homogeneous(),
                  origin: origin.to_homogeneous(),
                  distance_to_origin: 0.0,
                  inside_counter: 0,
                  depth_counter: 0 }
    }

    pub fn continue_ray_from_intersection(intersection: &RayIntersection, direction: VectorColumn4) -> Self {
        Self    { direction: direction,
                  origin: *intersection.get_intersection_point(),
                  distance_to_origin: intersection.get_distance_to_intersection() + intersection.get_ray_travel_distance(),
                  inside_counter: intersection.get_ray_inside_counter(),
                  depth_counter: intersection.get_ray_depth_counter() + 1}
    }

    pub fn continue_ray_from_previous(previous_ray: &Ray, origin: VectorColumn4, direction: VectorColumn4) -> Self {
        Self    { direction : direction,
                  origin : origin,
                  depth_counter : previous_ray.depth_counter + 1,
                  ..*previous_ray}
    }

    pub fn new_reversed_ray(ray: &Ray) -> Self {
        Self    { direction: -ray.direction,
                  ..*ray }
    }

    pub fn get_origin(&self) -> &VectorColumn4 {
        &self.origin
    }

    pub fn get_direction(&self) -> &VectorColumn4 {
        &self.direction
    }

    pub fn get_distance_to_origin(&self) -> DefNumType {
        self.distance_to_origin
    }

    pub fn get_inside_counter(&self) -> i32 {
        self.inside_counter
    }

    pub fn is_ray_inside_object(&self) -> bool {
        self.inside_counter > 0
    }

    pub fn get_depth_counter(&self) -> i32 {
        self.depth_counter
    }

    pub fn enter_object(&mut self) {
        self.inside_counter += 1;
    }

    pub fn leave_object(&mut self) {
        if self.inside_counter > 0 {
            self.inside_counter -= 1;
        } else {
            panic!("Ray left object more times than entered");
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use tools::NewHomogeneus;
    use core::model::Model;
    use defs::*;

    #[test]
    fn test_ray_new1_assignment() {
        let test_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));

        assert_eq!(test_ray.get_distance_to_origin(), 0.0);
        assert_eq!(test_ray.get_inside_counter(), 0);
        assert_eq!(test_ray.get_depth_counter(), 0);
    }

    #[test]
    fn test_continue_ray_from_intersection() {
        struct TestModel {

        }

        impl Model for TestModel {
            #[allow(dead_code)]
            fn get_model_view_matrix(&self) -> Option<Matrix4> {None}
            #[allow(dead_code)]
            fn get_intersection<'ray, 'model> (&self, ray: &'ray Ray) -> Option<RayIntersection<'ray, 'model>> {None}
            #[allow(dead_code)]
            fn get_ambient_color(&self, intersection: &RayIntersection) -> Option<Color> {None}
            #[allow(dead_code)]
            fn get_duffuse_color(&self, intersection: &RayIntersection) -> Option<Color> {None}
            #[allow(dead_code)]
            fn get_specular_color(&self, intersection: &RayIntersection) -> Option<Color> {None}
            #[allow(dead_code)]
            fn get_fresnel_reflect_color(&self, intersection: &RayIntersection) -> Option<Color> {None}
            #[allow(dead_code)]
            fn get_fresnel_refract_color(&self, intersection: &RayIntersection) -> Option<Color> {None}
        }    

        let dummy_model = TestModel {};
        let initial_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let intersection = RayIntersection::new(Vector3::new_homogeneous(0.0, 0.0, 1.0),
                                                Point3::new_homogeneous(1.0, 0.0, 0.0),
                                                &initial_ray,
                                                &dummy_model);
        let continued_ray = Ray::continue_ray_from_intersection(&intersection, Vector3::new_homogeneous(0.0, 1.0, 0.0));

        assert_eq!(continued_ray.get_depth_counter(), 1);
        assert_eq!(continued_ray.get_distance_to_origin(), 1.0);
    }
}