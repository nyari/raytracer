use core::{Model, Material, RayIntersection, Ray, RayIntersectionError};
use defs::{Point3, Vector3};
use tools::{CompareWithTolerance};
use na;
use std;

pub struct SolidUnitSphere {
    material: Material
}

impl SolidUnitSphere {
    pub fn new(material: Material) -> Self {
        Self {  material: material}
    }
}

impl Model for SolidUnitSphere {
    fn get_intersection(&self, ray: &Ray) -> Option<RayIntersection> {
        let dir = ray.get_direction();
        let origin = ray.get_origin();

        let a = dir.x.powi(2) + dir.y.powi(2) + dir.z.powi(2);
        let b = 2.0 * (origin.x * dir.x + origin.y * dir.y + origin.z * dir.z);
        let c = origin.x.powi(2) + origin.y.powi(2) + origin.z.powi(2) - 1.0;

        let determinant = b.powi(2) - 4.0 * a * c;
        if determinant.less_eps(&0.0) {
            None
        } else {
            let t1 = (-b + determinant.sqrt()) / (2.0 * a);
            let t2 = (-b - determinant.sqrt()) / (2.0 * a);
            let result_calc = |t, inside| {
                let intersection_point = origin + dir * t;
                let normal = intersection_point - Point3::origin();
                match RayIntersection::new(normal, intersection_point, ray, self.material, inside) {
                    Ok(intersection) => Some(intersection),
                    Err(RayIntersectionError::NoRayTravelDistance) => None,
                    _ => panic!("Unhandled RayIntersectionError")
                }
            };

            if t1.is_sign_negative() && t2.is_sign_negative() {
                None
            } else if t1.is_sign_positive() && t2.is_sign_negative() {
                result_calc(t1, true)
            } else {
                match result_calc(t2, false) {
                    Some(result) => Some(result),
                    None => result_calc(t1, true),
                }
            }
        }
    }
}


pub struct SolidXYPlane {
    material: Material
}

impl SolidXYPlane {
    pub fn new(material: Material) -> Self {
        Self {  material: material}
    }
}

impl Model for SolidXYPlane {
    fn get_intersection(&self, ray: &Ray) -> Option<RayIntersection> {
        let origin = ray.get_origin();
        let dir = ray.get_direction();
        let normal = Vector3::new(0.0, 0.0, 1.0);

        let u = normal.dot(dir);
        if !u.near_zero_eps() {
            let s = normal.x * origin.x + normal.y * origin.y + normal.z * origin.z;
            let t = (-s) / u;
            if t.is_sign_positive() {
                let is_inside = na::angle(&normal, dir).less_eps(&std::f64::consts::FRAC_PI_2);
                let point = origin + dir * t;
                match RayIntersection::new(normal, point, ray, self.material, is_inside) {
                    Ok(intersection) => Some(intersection),
                    Err(RayIntersectionError::NoRayTravelDistance) => None,
                    _ => panic!("Unhandled RayIntersectionError"),
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

