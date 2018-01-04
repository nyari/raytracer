use defs::{Matrix4};
use core::{Ray, RayIntersection};


pub trait Model {
    fn get_intersection (&self, ray: & Ray) -> Option<RayIntersection>;
}


pub struct SimpleModelWrapper<T: Model> {
    wrapped_model: T,
}

impl<T:Model> SimpleModelWrapper<T> {
    fn new(model: T) -> Self {
        Self {wrapped_model: model}
    }
}

impl<T: Model> Model for SimpleModelWrapper<T> {
    fn get_intersection(&self, ray: & Ray) -> Option<RayIntersection<>> {
        self.wrapped_model.get_intersection(ray)
    }
}


pub struct ModelViewModelWrapper<T: Model> {
    wrapped_model: T,
    tf_matrix: Matrix4,
    inverse_tf_matrix: Matrix4,
    inverse_transposed_tf_matrix: Matrix4
}

impl<T: Model> ModelViewModelWrapper<T> {
    fn new(model: T, model_view_matrix: Matrix4) -> Self {
        Self {  wrapped_model: model,
                inverse_tf_matrix: model_view_matrix.try_inverse().expect("Uninvertable Model View Matrix"),
                inverse_transposed_tf_matrix: model_view_matrix.try_inverse().expect("Uninvertable Model View Matrix").transpose(),
                tf_matrix: model_view_matrix
        }
    } 
}

impl<T: Model> Model for ModelViewModelWrapper<T> {
    fn get_intersection(&self, ray: & Ray) -> Option<RayIntersection<>> {
        let transformed_ray = ray.get_transformed((&self.inverse_tf_matrix, &self.inverse_tf_matrix));

        match self.wrapped_model.get_intersection(&transformed_ray) {
            None => None,
            Some(transformed_intersection) => Some(transformed_intersection.get_transformed((&self.tf_matrix, &self.inverse_transposed_tf_matrix)))
        }
    }
}


pub trait Intersector {    
    fn get_intersections_reverse_ordered(&self, ray: &Ray) -> Vec<RayIntersection>;
    fn get_nearest_intersection(&self, ray: &Ray) -> Option<RayIntersection>;
}