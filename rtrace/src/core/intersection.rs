use defs::{DefNumType, Point3, Vector3, Matrix4};

use core::{Ray, Material};

use ::na;


pub struct RayIntersection {
    normal : Vector3,
    point : Point3,
    material_at_intersection : Material,
    distance_to_intersection : DefNumType,
    was_inside : bool,
    ray_travel_distance: DefNumType,
    ray_depth_count: i32,
    ray_inside_count: i32
}

impl RayIntersection {
    pub fn new(normal: Vector3, point: Point3, ray: & Ray, material: Material, was_inside: bool) -> Self {
        Self {  normal: normal, 
                point: point, 
                ray_travel_distance: ray.get_distance_to_origin(),
                ray_depth_count: ray.get_depth_counter(),
                ray_inside_count: ray.get_inside_counter(),
                material_at_intersection: material,
                distance_to_intersection: na::distance(ray.get_origin(), &point),
                was_inside: was_inside
             }
    }

    pub fn get_intersection_point(&self) -> &Point3 {
        &self.point
    }

    pub fn get_normal_vector(&self) -> &Vector3 {
        &self.normal
    }

    pub fn get_distance_to_intersection(&self) -> DefNumType {
        self.distance_to_intersection
    }

    pub fn get_ray_travel_distance(&self) -> DefNumType {
        self.ray_travel_distance
    }

    pub fn get_ray_depth_counter(&self) -> i32 {
        self.ray_depth_count
    }

    pub fn get_ray_inside_counter(&self) -> i32 {
        self.ray_inside_count
    }

    pub fn get_material(&self) -> &Material {
        &self.material_at_intersection
    }

    pub fn was_inside(&self) -> bool {
        self.was_inside
    }

    pub fn get_transformed(self, point_and_dir_mx: (&Matrix4, &Matrix4)) -> Self {
        let (point_tf_mx, vector_tf_mx) = point_and_dir_mx;

        let point = self.point.to_homogeneous();
        let normal = self.normal.to_homogeneous();

        Self    { point: Point3::from_homogeneous(point_tf_mx * point).expect("Unhomogeneous transformed point"),
                  normal: Vector3::from_homogeneous(vector_tf_mx * normal).expect("Unhomogeneous transformed vector"),
                  ..self
        }
    }
}


#[cfg(test)]
mod tests {

}