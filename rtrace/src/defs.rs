use num::Num;

pub type DefNumType = f64;

pub type VectorRow4 = super::na::core::Matrix1x4<DefNumType>;
pub type VectorColumn4 = super::na::core::Vector4<DefNumType>;

pub type Matrix4 = super::na::core::Matrix4<DefNumType>;

pub type Point3 = super::na::Point3<DefNumType>;
pub type Vector3 = super::na::core::Vector3<DefNumType>;

pub type Color = super::na::core::Matrix3x1<DefNumType>;