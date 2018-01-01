#[cfg(test)]

#[macro_use]
extern crate approx; // For the macro relative_eq!
extern crate nalgebra as na;
extern crate float_cmp as flcmp;
extern crate num;
extern crate num_traits as numt;

pub mod defs;
pub mod tools;
pub mod core;
pub mod basic;
