mod hw;

fn main() {
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_double_int32() {
        assert_eq!(hw::hw1::double_int32(5), 10);
    }

    #[test]
    fn double_int64() {
        assert_eq!(hw::hw1::double_int64(6), 12);
    }    

    #[test]
    fn double_float32() {
        assert_eq!(hw::hw1::double_float32(7.), 14.);
    }

    #[test]
    fn test_double_float64() {
        assert_eq!(hw::hw1::double_float64(7.), 14.);
    }

    #[test]
    fn test_int_plus_float_to_float() {
        assert_eq!(hw::hw1::int_plus_float_to_float(5, 7.), 12.);
    }

    #[test]
    fn test_int_plus_float_to_int() {
        assert_eq!(hw::hw1::int_plus_float_to_int(6, 15.), 21);
    }

    #[test]
    fn test_tuple_sum() {
        assert_eq!(hw::hw1::tuple_sum((5, 9)), 14);
    }

    #[test]
    fn test_array_sum() {
        assert_eq!(hw::hw1::array_sum([3,4,5]), 12);
    }

    #[test]
    fn test_array_sum_for() {
        assert_eq!(hw::hw1::array_sum_for([8,14,25]), 47);
    }

    #[test]
    fn test_array_sum_iter() {
        assert_eq!(hw::hw1::array_sum_iter([7,1,2]), 10);
    }
}