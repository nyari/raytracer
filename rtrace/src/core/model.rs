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

        self.tf_matrix = similarity.to_homogeneous() * self.tf_matrix;

        self.recalculate_cached_matrices(); 
    }

    pub fn scale_non_uniform(&mut self, scaling: Vector3) {
        let diagonal = {
            let mut result = scaling.to_homogeneous();
            result[(3, 0)] = 1.0;
            result
        };

        let similarity = Matrix4::from_diagonal(&diagonal);
        
        self.tf_matrix = similarity * self.tf_matrix;

        self.recalculate_cached_matrices();
    }

    pub fn translate(&mut self, translation: Vector3) {
        let translate = Translation3::from_vector(translation);

        self.tf_matrix = translate.to_homogeneous() * self.tf_matrix;

        self.recalculate_cached_matrices();
    }

    pub fn rotate(&mut self, axis: Vector3, angle: FloatType) {
        let rotation = Rotation3::from_axis_angle(&Unit::new_normalize(axis), angle);

        self.tf_matrix = rotation.to_homogeneous() * self.tf_matrix;

        self.recalculate_cached_matrices();
    }

    #[cfg(test)]
    pub fn get_tf_matrix(&self) -> &Matrix4 {
        &self.tf_matrix
    }
}

impl<T: Model> Model for ModelViewModelWrapper<T> {
    fn get_intersection(&self, ray: & Ray) -> Option<RayIntersection<>> {
        let transformed_ray = ray.get_transformed((&self.inverse_tf_matrix, &self.inverse_tf_matrix));

        match self.wrapped_model.get_intersection(&transformed_ray) {
            None => None,
            Some(transformed_intersection) => Some(transformed_intersection.get_transformed((&self.tf_matrix, &self.tf_matrix)))
        }
    }
}


pub trait Intersector {    
    fn get_intersections_reverse_ordered(&self, ray: &Ray) -> Vec<RayIntersection>;
    fn get_nearest_intersection(&self, ray: &Ray) -> Option<RayIntersection>;
}


#[cfg(test)]
mod tests {
    use super::*;
    use core::{Material};
    use defs::{Point3};
    use std;

    struct DummyModel {

    }

    impl DummyModel {
        pub fn new() -> Self {
            Self {

            }
        }
    }

    impl Model for DummyModel {
         fn get_intersection(&self, ray: &Ray) -> Option<RayIntersection> {
             None
         }
    }

    #[test]
    fn mvo_wrapper_scale_uniform() {
        let mut wrapper = ModelViewModelWrapper::new_identity(DummyModel::new());
        wrapper.scale_uniform(2.0);

        let expected = Matrix4::new(2.0,    0.0,    0.0,    0.0,
                                    0.0,    2.0,    0.0,    0.0,
                                    0.0,    0.0,    2.0,    0.0,
                                    0.0,    0.0,    0.0,    1.0);
        
        assert_relative_eq!(wrapper.get_tf_matrix(), &expected);
    }

    #[test]
    fn mvo_wrapper_scale_non_uniform() {
        let mut wrapper = ModelViewModelWrapper::new_identity(DummyModel::new());
        wrapper.scale_non_uniform(Vector3::new(2.0, 3.0, 4.0));

        let expected = Matrix4::new(2.0,    0.0,    0.0,    0.0,
                                    0.0,    3.0,    0.0,    0.0,
                                    0.0,    0.0,    4.0,    0.0,
                                    0.0,    0.0,    0.0,    1.0);
        
        assert_relative_eq!(wrapper.get_tf_matrix(), &expected);
    }

    #[test]
    fn mvo_wrapper_translate() {
        let mut wrapper = ModelViewModelWrapper::new_identity(DummyModel::new());
        wrapper.translate(Vector3::new(1.0, 1.0, 1.0));

        let expected = Matrix4::new(1.0,    0.0,    0.0,    1.0,
                                    0.0,    1.0,    0.0,    1.0,
                                    0.0,    0.0,    1.0,    1.0,
                                    0.0,    0.0,    0.0,    1.0);
        
        assert_relative_eq!(wrapper.get_tf_matrix(), &expected);
    }

    #[test]
    fn mvo_wrapper_scale_translate() {
        let mut wrapper = ModelViewModelWrapper::new_identity(DummyModel::new());

        wrapper.scale_non_uniform(Vector3::new(2.0, 2.0, 2.0));
        wrapper.translate(Vector3::new(1.0, 1.0, 1.0));

        let expected = Matrix4::new(2.0,    0.0,    0.0,    1.0,
                                    0.0,    2.0,    0.0,    1.0,
                                    0.0,    0.0,    2.0,    1.0,
                                    0.0,    0.0,    0.0,    1.0);
        
        assert_relative_eq!(wrapper.get_tf_matrix(), &expected);
    }

    struct ModelMock {
        point: Point3,
        normal: Vector3,
    }

    impl ModelMock {
        pub fn new(point: Point3, normal: Vector3) -> Self {
            Self {  point: point,
                    normal: normal  }
        }
    }

    impl Model for ModelMock {
        fn get_intersection(&self, ray: &Ray) -> Option<RayIntersection> {
            Some(RayIntersection::new(self.normal, self.point, ray, Material::new_useless(), false).expect("Ray depth limit reached"))
        }
    }

    #[test]
    fn mvo_wrapper_complete_translate() {
        let mut test_model = ModelViewModelWrapper::new_identity(ModelMock::new(Point3::new(0.0, 0.0, 1.0), Vector3::new(1.0, -1.0, 1.0)));
        test_model.translate(Vector3::new(0.0, 0.0, 1.0));

        let intersector_ray = Ray::new_single_shot(Point3::new(0.0, -5.0, 2.0), Vector3::new(0.0, 1.0, 0.0));
        let transformed_intersection = test_model.get_intersection(&intersector_ray).expect("There was no intersection returned when ModelMock always returns");

        assert_relative_eq!(transformed_intersection.get_intersection_point(), &Point3::new(0.0, 0.0, 2.0));
        assert_relative_eq!(transformed_intersection.get_normal_vector(), &Vector3::new(1.0, -1.0, 1.0).normalize());
    }

    #[test]
    fn mvo_wrapper_complete_rotate_translate() {
        let mut test_model = ModelViewModelWrapper::new_identity(ModelMock::new(Point3::new(0.0, 0.0, 1.0), Vector3::new(1.0, -1.0, 1.0)));
        test_model.translate(Vector3::new(0.0, 0.0, 1.0));
        test_model.rotate(Vector3::new(0.0, 0.0, 1.0), std::f64::consts::FRAC_PI_2);

        let intersector_ray = Ray::new_single_shot(Point3::new(0.0, -5.0, 2.0), Vector3::new(0.0, 1.0, 0.0));
        let transformed_intersection = test_model.get_intersection(&intersector_ray).expect("There was no intersection returned when ModelMock always returns");

        assert_relative_eq!(transformed_intersection.get_intersection_point(), &Point3::new(0.0, 0.0, 2.0));
        assert_relative_eq!(transformed_intersection.get_normal_vector(), &Vector3::new(1.0, 1.0, 1.0).normalize());
    }

    #[test]
    fn mvo_wrapper_complete_scale_translate() {
        let mut test_model = ModelViewModelWrapper::new_identity(ModelMock::new(Point3::new(0.0, 0.0, 1.0), Vector3::new(1.0, -1.0, 1.0)));
        test_model.scale_uniform(2.0);
        test_model.translate(Vector3::new(0.0, 1.0, 0.0));        

        let intersector_ray = Ray::new_single_shot(Point3::new(0.0, -5.0, 2.0), Vector3::new(0.0, 1.0, 0.0));
        let transformed_intersection = test_model.get_intersection(&intersector_ray).expect("There was no intersection returned when ModelMock always returns");

        assert_relative_eq!(transformed_intersection.get_intersection_point(), &Point3::new(0.0, 1.0, 2.0));
        assert_relative_eq!(transformed_intersection.get_normal_vector(), &Vector3::new(1.0, -1.0, 1.0).normalize());
    }
}