use defs::{Matrix4, Vector3, FloatType};
use core::{Ray, RayIntersection};
use na::{Similarity3, Rotation3, Translation3, Unit};

pub trait Model {
    fn get_intersection(&self, ray: &Ray) -> Option<RayIntersection>;
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
    pub fn new(model: T, model_view_matrix: Matrix4) -> Self {
        Self {  wrapped_model: model,
                inverse_tf_matrix: model_view_matrix.try_inverse().expect("Uninvertable Model View Matrix"),
                inverse_transposed_tf_matrix: model_view_matrix.try_inverse().expect("Uninvertable Model View Matrix").transpose(),
                tf_matrix: model_view_matrix
        }
    }

    pub fn new_identity(model: T) -> Self {
        Self {  wrapped_model: model,
                inverse_tf_matrix: Matrix4::identity(),
                inverse_transposed_tf_matrix: Matrix4::identity(),
                tf_matrix: Matrix4::identity()
        }
    }

    fn recalculate_cached_matrices(&mut self) {
        self.inverse_tf_matrix = self.tf_matrix.try_inverse().expect("Uninvertable Model View Matrix");
        self.inverse_transposed_tf_matrix = self.inverse_tf_matrix.transpose();
    }

    pub fn load_identity(&mut self) {
        self.tf_matrix = Matrix4::identity();
        self.inverse_tf_matrix = Matrix4::identity();
        self.inverse_transposed_tf_matrix = Matrix4::identity();
    }

    pub fn scale_uniform(&mut self, scaling: FloatType) {
        let similarity = Similarity3::from_scaling(scaling);

        self.tf_matrix = self.tf_matrix * similarity.to_homogeneous();

        self.recalculate_cached_matrices(); 
    }

    pub fn scale_non_uniform(&mut self, scaling: Vector3) {
        let diagonal = scaling.to_homogeneous();
        let similarity = Matrix4::from_diagonal(&diagonal);
        
        self.tf_matrix = self.tf_matrix * similarity;

        self.recalculate_cached_matrices();
    }

    pub fn translate(&mut self, translation: Vector3) {
        let translate = Translation3::from_vector(translation);

        self.tf_matrix = self.tf_matrix * translate.to_homogeneous();

        self.recalculate_cached_matrices();
    }

    pub fn rotate(&mut self, axis: Vector3, angle: FloatType) {
        let rotation = Rotation3::from_axis_angle(&Unit::new_normalize(axis), angle);

        self.tf_matrix = self.tf_matrix * rotation.to_homogeneous();

        self.recalculate_cached_matrices();
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