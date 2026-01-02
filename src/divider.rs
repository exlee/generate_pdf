use std::vec::Vec;

pub fn divide(size: usize, pages: usize) -> Vec<usize> {
    let mut result: Vec<usize> = Vec::new();
    result.reserve_exact(pages);

    let divider = size/pages;
    for _ in  (0..pages - 1).into_iter() {
        result.push(divider)
    }
    result.push(size - ((pages-1) * divider));

    return result;
}

pub fn divide_at(size: usize, pages: usize, at: usize) -> usize {
    divide(size,pages)[at]
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(divide(10,1), vec![10])
    }

    #[test]
    fn basic2() {
        assert_eq!(divide(33,3), vec![11,11,11])
    }

    #[test]
    fn basic3() {
        assert_eq!(divide(34,3), vec![11,11,12])
    }

    #[test]
    fn basic4() {
        assert_eq!(divide(10,3), vec![3,3,4])
    }

    #[test]
    fn basic5() {
        assert_eq!(divide(555,1), vec![555])
    }


    #[test]
    fn divider_smaller_than_size() {
        assert_eq!(divide(1,3), vec![0,0,1])
    }

    #[test]
    fn dumps_value_when_divider_is_1() {
        assert_eq!(divide(1,1), vec![1]);
    }

    #[test]
    #[should_panic]
    fn panics_when_divider_is_0() {
        divide(1,0);
    }

}
