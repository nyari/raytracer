pub mod conv {
    use defs::VectorTup;
    use defs::PointTup;
    use defs::VectorColumn4;

    pub trait ToVectorColumn4 {
        fn tovectcolumn4(&self) -> VectorColumn4;
    }

    impl ToVectorColumn4 for VectorTup {
        fn tovectcolumn4(&self) -> VectorColumn4 {
            let (x, y, z) = self.get();       
            VectorColumn4::new(x, y, z, 0.0)
        }
    }

    impl ToVectorColumn4 for PointTup {
        fn tovectcolumn4(&self) -> VectorColumn4 {  
            let (x, y, z) = self.get();   
            VectorColumn4::new(x, y, z, 1.0)
        }
    }

    pub fn vectcolumn4<T: ToVectorColumn4> (data: T) -> VectorColumn4 {
        data.tovectcolumn4()
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use defs::DefNumType;
        use defs::VectorTup;
        use defs::PointTup;

        #[test]
        fn test_vectcolumn4() {
            let mut buffer: Vec<DefNumType> = Vec::new();

            let from_tuple_vector: VectorTup = VectorTup::new(1.0, 2.0, 3.0);
            let from_vector = vectcolumn4(from_tuple_vector);
            let from_vector_reference = vec![1.0, 2.0, 3.0, 0.0];
            for element in from_vector.iter() {
                buffer.push(element.clone());
            }
            assert_eq!(buffer, from_vector_reference);

            buffer.clear();

            let from_tuple_point:PointTup = PointTup::new(1.0, 2.0, 3.0);
            let from_point = vectcolumn4(from_tuple_point);
            let from_point_reference = vec![1.0, 2.0, 3.0, 1.0];
            for element in from_point.iter() {
                buffer.push(element.clone());
            }
            assert_eq!(buffer, from_point_reference);
        }
    }
}