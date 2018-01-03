use defs::{Vector3, Point3, DefNumType, Matrix4};
use core::RayIntersection;

#[derive(Debug)]
pub struct Ray {
    direction : Vector3,
    origin : Point3,
    distance_to_origin : DefNumType,
    inside_counter : i32,
    depth_counter : i32
}


impl Ray {
    pub fn new(origin: Point3, dir: Vector3) -> Self {
        Self    { direction: dir.normalize(),
                  origin: origin,
                  distance_to_origin: 0.0,
                  inside_counter: 0,
                  depth_counter: 0 }
    }

    pub fn continue_ray_from_intersection(intersection: &RayIntersection, direction: Vector3) -> Self {
        Self    { direction: direction.normalize() ,
                  origin: *intersection.get_intersection_point(),
                  distance_to_origin: intersection.get_distance_to_intersection() + intersection.get_ray_travel_distance(),
                  inside_counter: intersection.get_ray_inside_counter(),
                  depth_counter: intersection.get_ray_depth_counter() + 1}
    }

    pub fn continue_ray_from_previous(previous_ray: &Ray, origin: Point3, direction: Vector3) -> Self {
        Self    { direction : direction.normalize(),
                  origin : origin,
                  depth_counter : previous_ray.depth_counter + 1,
                  ..*previous_ray}
    }

    pub fn new_reversed_ray(ray: &Ray) -> Self {
        Self    { direction: (-ray.direction).normalize(),
                  ..*ray }
    }

    pub fn get_transformed(&self, point_and_dir_mx: (&Matrix4, &Matrix4)) -> Self {
        let (point_tf_mx, vector_tf_mx) = point_and_dir_mx;

        let origin = self.origin.to_homogeneous();
        let direction = self.direction.to_homogeneous();

        Self    { origin: Point3::from_homogeneous(point_tf_mx * origin).expect("Unhomogeneous transformed point"),
                  direction: Vector3::from_homogeneous(vector_tf_mx * direction).expect("Unhomogeneous transformed vector"),
                  ..*self
        }
    }

    pub fn get_origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vector3 {
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
    use core::material::Material;
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
                                                false);
        let continued_ray = Ray::continue_ray_from_intersection(&intersection, Vector3::new(0.0, 1.0, 0.0));

        assert_eq!(continued_ray.get_depth_counter(), 1);
        assert_eq!(continued_ray.get_distance_to_origin(), 1.0);
    }
}
