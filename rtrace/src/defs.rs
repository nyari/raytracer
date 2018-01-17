pub type FloatType = f64;
pub type IntType = i32;

pub type VectorRow4 = super::na::core::Matrix1x4<FloatType>;
pub type VectorColumn4 = super::na::core::Vector4<FloatType>;

pub type Matrix4 = super::na::core::Matrix4<FloatType>;

pub type Point3 = super::na::Point3<FloatType>;
pub type Vector3 = super::na::core::Vector3<FloatType>;

pub type Point2 = super::na::Point2<FloatType>;
pub type Vector2 = super::na::Vector2<FloatType>;

pub type Point2Int = super::na::Point2<IntType>;
pub type Vector2Int = super::na::Vector2<IntType>;

pub static FLOAT_ULPS_TOLERANCE: i64 = 65536;