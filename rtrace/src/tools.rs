pub mod conv {
    use defs::VectorTup;
    use defs::PointTup;
    use defs::VectorRow4;

    pub trait ToVectorRow4 {
        fn tovectrow4(&self) -> VectorRow4;
    }

    impl ToVectorRow4 for VectorTup {
        fn tovectrow4(&self) -> VectorRow4 {        
            VectorRow4::new(self.x, self.y, self.z, 0.0)
        }
    }

    impl ToVectorRow4 for PointTup {
        fn tovectrow4(&self) -> VectorRow4 {            
            VectorRow4::new(self.x, self.y, self.z, 1.0)
        }
    }

    pub fn vectrow4<T: ToVectorRow4> (data: T) -> VectorRow4 {
        data.tovectrow4()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use defs::DefNumType;
        use defs::VectorTup;
        use defs::PointTup;

        #[test]
        fn test_vectrow4() {
            let mut buffer: Vec<DefNumType> = Vec::new();

            let from_tuple_vector: VectorTup = VectorTup {x:1.0, y:2.0, z:3.0};
            let from_vector = vectrow4(from_tuple_vector);
            let from_vector_reference = vec![1.0, 2.0, 3.0, 0.0];
            for element in from_vector.iter() {
                buffer.push(element.clone());
            }
            assert_eq!(buffer, from_vector_reference);

            buffer.clear();

            let from_tuple_point:PointTup = PointTup {x:1.0, y:2.0, z:3.0};
            let from_point = vectrow4(from_tuple_point);
            let from_point_reference = vec![1.0, 2.0, 3.0, 1.0];
            for element in from_point.iter() {
                buffer.push(element.clone());
            }
            assert_eq!(buffer, from_point_reference);
        }
    }
}