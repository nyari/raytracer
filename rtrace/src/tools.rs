use std::cmp::Ordering;
use defs::{Vector3, FloatType, IntType, FLOAT_ULPS_TOLERANCE};
use ::na;
use ::numt::Num;
use ::numt::float::Float;
use ::flcmp::ApproxOrdUlps;


pub trait CompareWithTolerance<T: Float> {
    fn compare_eps(&self, rhs: &T) -> Ordering;
    fn less_eps(&self, rhs: &T) -> bool;
    fn less_eq_eps(&self, rhs: &T) -> bool;
    fn equal_eps(&self, rhs: &T) -> bool;
    fn greater_eps(&self, rhs: &T) -> bool;
    fn greater_eq_eps(&self, rhs: &T) -> bool;
    fn near_zero_eps(&self) -> bool;
}

impl CompareWithTolerance<FloatType> for FloatType {
    fn compare_eps(&self, rhs: &FloatType) -> Ordering {
        self.approx_cmp(&rhs, FLOAT_ULPS_TOLERANCE)
    }

    fn less_eps(&self, rhs: &FloatType) -> bool {
        self.approx_cmp(&rhs, FLOAT_ULPS_TOLERANCE) == Ordering::Less
    }

    fn less_eq_eps(&self, rhs: &FloatType) -> bool {
        match self.approx_cmp(&rhs, FLOAT_ULPS_TOLERANCE) {
            Ordering::Less => true,
            Ordering::Equal => true,
            _ => false,
        }
    }

    fn equal_eps(&self, rhs: &FloatType) -> bool {
        self.approx_cmp(&rhs, FLOAT_ULPS_TOLERANCE) == Ordering::Equal
    }

    fn greater_eps(&self, rhs: &FloatType) -> bool {
        self.approx_cmp(&rhs, FLOAT_ULPS_TOLERANCE) == Ordering::Greater
    }

    fn greater_eq_eps(&self, rhs: &FloatType) -> bool {
        match self.approx_cmp(&rhs, FLOAT_ULPS_TOLERANCE) {
            Ordering::Greater => true,
            Ordering::Equal => true,
            _ => false,
        }
    }

    fn near_zero_eps(&self) -> bool {
        self.approx_cmp(&0.0, FLOAT_ULPS_TOLERANCE) == Ordering::Equal
    }
}


pub trait Vector3Extensions {
    fn same_direction_as(&self, rhs: &Vector3) -> bool;
    fn length(&self) -> FloatType;
}

impl Vector3Extensions for Vector3 {
    fn same_direction_as(&self, rhs: &Vector3) -> bool {
        let closed_angle = na::angle(self, &rhs);
        closed_angle.near_zero_eps()
    }

    fn length(&self) -> FloatType {
        let mut result: FloatType = 0.0;

        for item in self.iter() {
            result += item.powi(2);
        }

        result.sqrt()
    }
}


pub trait OrderingQuery {
    fn is_less(self) -> bool;
    fn is_equal(self) -> bool;
    fn is_greater(self) -> bool;
}

impl OrderingQuery for Ordering {
    fn is_less(self) -> bool {
        match self {
            Ordering::Less => true,
            _ => false
        }
    }

    fn is_equal(self) -> bool {
        match self {
            Ordering::Equal => true,
            _ => false
        }
    }

    fn is_greater(self) -> bool {
        match self {
            Ordering::Greater => true,
            _ => false
        }
    }
}

pub trait Between<T: Num> {
    fn between(&self, less: &T, more: &T) -> bool;
}

impl Between<FloatType> for FloatType {
    fn between(&self, less: &FloatType, more: &FloatType) -> bool {
        less.greater_eq_eps(&self) && more.less_eq_eps(&self)
    }
}

impl Between<IntType> for IntType {
    fn between(&self, less: &IntType, more: &IntType) -> bool {
        less <= self && self <= more
    }
}