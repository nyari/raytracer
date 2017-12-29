// use defs::VectorTup;
// use defs::PointTup;
// use defs::DefNumType;
// use defs::VectorColumn4;
// use defs::Matrix4;
// use defs::Color;

// use core::ray::Ray;
// use core::intersection::RayIntersection;
use core::model::Model;


pub struct World {
    models : Vec<Box<Model + 'static>>,
}