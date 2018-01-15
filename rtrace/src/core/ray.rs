use defs::{Vector3, Point3, FloatType, Matrix4};
use core::RayIntersection;
use tools::Vector3Extensions;
use na::{Unit};


#[derive(Debug)]
pub enum RayError {
    DepthLimitReached,
    InvalidContinuationDirection,
}


#[derive(Clone, Copy, Debug)]
pub struct RayState {
    distance_to_origin : FloatType,
    inside_counter : i32,
    depth_counter : i32,
    depth_limit: Option<u32>,
}

impl RayState {
    pub fn get_continuation(input: &Self, distance_to_intersection: FloatType) -> Result<Self, RayError> {
        match input.depth_limit {
            Some(depth_limit) => {
                if depth_limit >= 1 {
                    Ok (Self {  distance_to_origin: input.distance_to_origin + distance_to_intersection,
                                depth_counter: input.depth_counter + 1,
                                depth_limit: Some(depth_limit - 1),
                                ..*input
                    })
                } else {
                    Err(RayError::DepthLimitReached)
                }
            },
            None => {
                    Ok (Self {  distance_to_origin: input.distance_to_origin + distance_to_intersection,
                                depth_counter: input.depth_counter + 1,
                                ..*input
                    })
            }
        }
    }

    pub fn get_distance_to_origin(&self) -> FloatType {
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


#[derive(Debug, Clone, Copy)]
pub struct Ray {
    direction : Unit<Vector3>,
    origin : Point3,
    state : RayState
}

impl Ray {
    pub fn new(origin: Point3, dir: Vector3) -> Self {
        Self    { direction: Unit::new_normalize(dir),
                  origin: origin,
                  state: RayState { distance_to_origin: 0.0,
                                    inside_counter: 0,
                                    depth_counter: 0,
                                    depth_limit: None }
        }
    }

    pub fn new_depth_limited(origin: Point3, dir: Vector3, depth_limit: u32) -> Self {
        Self    { direction: Unit::new_normalize(dir),
                  origin: origin,
                  state: RayState { distance_to_origin: 0.0,
                                    inside_counter: 0,
                                    depth_counter: 0,
                                    depth_limit: Some(depth_limit) }
        }
    }

    pub fn new_single_shot(origin: Point3, dir: Vector3) -> Self {
        Self::new_depth_limited(origin, dir, 1)
    }

    pub fn continue_ray_from_intersection(intersection: &RayIntersection, direction: Vector3) -> Result<Self, RayError> {
        match RayState::get_continuation(intersection.get_itersector_ray().get_state(), intersection.get_distance_to_intersection()) {
            Ok (continued_state) => {
                Ok (Self {  direction: Unit::new_normalize(direction),
                            origin: *intersection.get_intersection_point(),
                            state: continued_state})
            },
            Err(e) => Err(e)
        }
    }

    pub fn continue_ray_from_previous(previous_ray: &Ray, origin: Point3, direction: Vector3) -> Result<Self, RayError> {
        let calculated_direction = origin - previous_ray.get_origin();

        match RayState::get_continuation(&previous_ray.state, calculated_direction.length()) {
            Ok (continued_state) => {
                Ok (Self {  direction: Unit::new_normalize(direction),
                            origin: origin,
                            state: continued_state})
            },
            Err(e) => Err(e)
        }
    }

    pub fn new_reversed_ray(ray: &Ray) -> Self {
        Self    { direction: (-ray.direction),
                  ..*ray }
    }

    pub fn get_transformed(&self, point_and_dir_mx: (&Matrix4, &Matrix4)) -> Self {
        let (point_tf_mx, vector_tf_mx) = point_and_dir_mx;

        let origin = self.origin.to_homogeneous();
        let direction = self.direction.to_homogeneous();

        Self    { origin: Point3::from_homogeneous(point_tf_mx * origin).expect("Unhomogeneous transformed point"),
                  direction: Unit::new_normalize(Vector3::from_homogeneous(vector_tf_mx * direction).expect("Unhomogeneous transformed vector")),
                  ..*self
        }
    }

    pub fn get_origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vector3 {
        &self.direction.as_ref()
    }

    pub fn get_distance_to_origin(&self) -> FloatType {
        self.state.get_distance_to_origin()
    }

    pub fn get_inside_counter(&self) -> i32 {
        self.state.get_inside_counter()
    }

    pub fn is_ray_inside_object(&self) -> bool {
        self.state.is_ray_inside_object()
    }

    pub fn get_depth_counter(&self) -> i32 {
        self.state.get_depth_counter()
    }

    pub fn enter_object(&mut self) {
        self.state.enter_object()
    }

    pub fn leave_object(&mut self) {
        self.state.leave_object()
    }

    pub fn get_state(&self) -> &RayState {
        &self.state
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use core::{RayIntersection, RayIntersectionError, Material};
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
        let initial_ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        let intersection = RayIntersection::new(Vector3::new(0.0, 0.0, 1.0),
                                                Point3::new(1.0, 0.0, 0.0),
                                                &initial_ray,
                                                Material::new_useless(),
                                                false).unwrap();
        let continued_ray = Ray::continue_ray_from_intersection(&intersection, Vector3::new(0.0, 1.0, 0.0)).unwrap();

        assert_eq!(continued_ray.get_depth_counter(), 1);
        assert_eq!(continued_ray.get_distance_to_origin(), 1.0);
    }

    #[test]
    fn test_depth_limits() {
        let initial_ray = Ray::new_depth_limited(Point3::new(0.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0), 1);
        let continue_1 = Ray::continue_ray_from_previous(&initial_ray, Point3::new(1.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0)).expect("Ray should still continue");
        let continue_2 = Ray::continue_ray_from_previous(&continue_1, Point3::new(2.0, 0.0, 0.0), Vector3::new(1.0, 0.0, 0.0));
        if !continue_2.is_err() {
            panic!("Ray should not continue further");
        }
    }
}
