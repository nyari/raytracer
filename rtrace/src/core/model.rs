use core::{Ray, RayIntersection};

pub trait Model {
    fn get_intersection<'ray> (&self, ray: &'ray Ray) -> Option<RayIntersection<'ray>>;
}