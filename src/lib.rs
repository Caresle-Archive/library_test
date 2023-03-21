mod vector;

use vector::{Point, sum_point};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn point_2() {
        let result = Point { x: 20.0, y: 30.0 };
        assert_eq!(result.x, 20.0);
    }

    #[test]
    fn sum_point_2() {
        let sum = sum_point(Point { x: 10.0, y: 20.0}, Point { x: 30.0, y: 6.0 });
        assert_eq!(sum.x, 40.0);
        assert_eq!(sum.y, 26.0);
    }
}
