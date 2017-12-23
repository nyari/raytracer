pub type DefNumType = f64;

pub struct VectorTupBase<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub struct PointTupBase<T>  {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type VectorTup = VectorTupBase<DefNumType>;
pub type PointTup = PointTupBase<DefNumType>;

pub type VectorRow4 = super::na::core::Matrix1x4<DefNumType>;
pub type VectorColumn4 = super::na::core::Matrix4x1<DefNumType>;

pub type Matrix4 = super::na::core::Matrix4<DefNumType>;