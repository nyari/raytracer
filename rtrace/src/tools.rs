use defs::Vector3;
use defs::Point3;
use defs::VectorColumn4;
use defs::DefNumType;

pub trait NewHomogeneus {
    fn new_homogeneous(x: DefNumType, y: DefNumType, z: DefNumType) -> VectorColumn4;
}

impl NewHomogeneus for Vector3 {
    fn new_homogeneous(x: DefNumType, y: DefNumType, z: DefNumType) -> VectorColumn4 {
        Vector3::new(x, y, z).to_homogeneous()
    }
}

impl NewHomogeneus for Point3 {
   fn new_homogeneous(x: DefNumType, y: DefNumType, z: DefNumType) -> VectorColumn4 {
       Point3::new(x, y, z).to_homogeneous()
   }
}