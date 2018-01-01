use std::cmp::Ordering;
use defs::{Vector3, DefNumType, FLOAT_ULPS_TOLERANCE};
use ::na;
use ::numt::float::Float;
use ::flcmp::ApproxOrdUlps;


pub trait CompareWithTolerance<T: Float> {
    fn compare_eps(&self, rhs: &T) -> Ordering;
    fn near_zero_eps(&self) -> bool;
}

impl CompareWithTolerance<DefNumType> for DefNumType {
    fn compare_eps(&self, rhs: &DefNumType) -> Ordering {
        self.approx_cmp(&rhs, FLOAT_ULPS_TOLERANCE)
    }

    fn near_zero_eps(&self) -> bool {
        self.approx_cmp(&0.0, FLOAT_ULPS_TOLERANCE) == Ordering::Equal
    }
}


pub trait Vector3Extensions {
    fn same_direction_as(&self, rhs: &Vector3) -> bool;
    fn length(&self) -> DefNumType;
}

impl Vector3Extensions for Vector3 {
    fn same_direction_as(&self, rhs: &Vector3) -> bool {
        let closed_angle = na::angle(self, &rhs);
        closed_angle.near_zero_eps()
    }

    fn length(&self) -> DefNumType {
        let mut result: DefNumType = 0.0;

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