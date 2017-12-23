mod defs;
mod tools;
mod tools::convto;

use defs::VectorTup;
use defs::PointTup;
use defs::defnum_t;
use defs::VectorRow4;

use defs::VectorRowCount;
use defs::VectorColumnCount;

use tools::convto;

#[derive(Debug)]
pub struct RayBase<T: num::Num> {
    direction : VectorRow4<T>,
    origin : VectorRow4<T>,
    distance_to_origin : T,
    inverted : bool
}

pub type Ray = RayBase<defnum_t>;

impl<T : num::Num> RayBase<T> {
    fn new(origin: PointTup, dir: VectorTup) -> RayBase<T> {
        RayBase {convto::vectrow4(dir), convto::vectrow4(origin), 0.0, false}
    }

    fn get_origin(&self) -> VectorRow4<T> {
        return self.origin;
    }

    fn get_direction(&self) -> VectorRow4<T> {
        return self.direction;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ray_new1_assignment() {
        let test_ray = Ray::new(PointTup(0.0, 0.0, 0.0), VectorTup(1.0, 0.0, 0.0));

        let read_dir = test_ray.get_direction();
        let read_origin = test_ray.get_origin();
        assert_eq!(read_dir, );
        assert_eq!(read_origin, Point(0.0, 0.0, 0.0));
    }
}