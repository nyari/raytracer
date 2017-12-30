use num::Num;

pub type DefNumType = f64;

pub struct VectorTupBase<T: Clone + Num> {
    x: T,
    y: T,
    z: T,
}

impl<T: Clone + Num> VectorTupBase<T> {
    pub fn new (x: T, y: T, z: T) -> Self {
        Self {x: x, y: y, z: z}
    }

    pub fn get(&self) -> (T, T, T) {
        (self.x.clone(), self.y.clone(), self.z.clone())
    }
}

pub struct PointTupBase<T: Clone + Num>  {
    x: T,
    y: T,
    z: T,
}

impl<T: Clone + Num> PointTupBase<T> {
    pub fn new (x: T, y: T, z: T) -> Self {
        Self {x: x, y: y, z: z}
    }

    pub fn get(&self) -> (T, T, T) {
        (self.x.clone(), self.y.clone(), self.z.clone())
    }
}

pub type VectorTup = VectorTupBase<DefNumType>; 
pub type PointTup = PointTupBase<DefNumType>;

pub type VectorRow4 = super::na::core::Matrix1x4<DefNumType>;
pub type VectorColumn4 = super::na::core::Vector4<DefNumType>;

pub type Matrix4 = super::na::core::Matrix4<DefNumType>;

pub type Point3 = super::na::Point3<DefNumType>;
pub type Vector3 = super::na::core::Vector3<DefNumType>;

pub type Color = super::na::core::Matrix3x1<DefNumType>;