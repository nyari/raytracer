use core::*;

pub mod intersector;
pub mod illuminator;
pub mod colorcalculator;

pub use self::intersector::*;
pub use self::illuminator::*;
pub use self::colorcalculator::*;


pub type SimpleWorld = World<SimpleIntersector, SimpleColorCalculator, SimpleIlluminator>;