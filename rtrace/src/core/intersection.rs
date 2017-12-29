use defs::VectorColumn4;

use core::Ray;
use core::Model;

#[derive(Debug)]
pub struct RayIntersection<'ray> {
    normal : VectorColumn4,
    point : VectorColumn4,
    itersector_ray : &'ray Ray,
}

// impl<'ray> RayIntersection<'ray> {

// }


#[cfg(test)]
mod tests {

    #[test]
    fn test_model_ray_intersecton() {

    }

}