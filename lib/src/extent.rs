use std::cmp::PartialOrd;

type ValueOfFn<T> = Box<dyn Fn(T, T, T) -> T>;

/// Return the min and max simultaneously.
pub fn extent<T>(values: Vec<T>, value_of: &Option<ValueOfFn<T>>) -> [T; 2]
where
    T: PartialOrd + Copy,
{
    let mut min: Option<T> = None;
    let mut max: Option<T> = None;
    match value_of {
        None => {
            for value in values {
                match min {
                    None => {
                        min = Some(value);
                    }
                    Some(min_val) => {
                        if min_val > value {
                            min = Some(value);
                        }
                    }
                }
                match max {
                    None => {
                        max = Some(value);
                    }
                    Some(max_val) => {
                        if max_val < value {
                            max = Some(value);
                        }
                    }
                }
            }
        }
        Some(_) => {
            unimplemented!("Not yet supported: extent() valueof function parameter.");
        }
    }

    [min.unwrap(), max.unwrap()]
}

#[cfg(test)]
mod extent_test {
    extern crate pretty_assertions;

    use pretty_assertions::assert_eq;

    use crate::extent::extent;

    #[test]
    fn returns_the_least_and_greatest_numeric() {
        println!("extent(array) returns the least and greatest numeric values for numbers");
        assert_eq!(extent(vec![1], &None), [1, 1]);
        assert_eq!(extent(vec![1], &None), [1, 1]);
        assert_eq!(extent(vec![5, 1, 2, 3, 4], &None), [1, 5]);
        assert_eq!(extent(vec![20, 3], &None), [3, 20]);
        assert_eq!(extent(vec![3, 20], &None), [3, 20]);
    }
}
