struct Point2D {
    x: i32,
    y: i32,
}

impl Point2D {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //     #[test]
    //     fn point_equality() {
    //         assert_eq!(Point2D::new(1, 2), Point2D::new(1, 2));
    //         assert_ne!(Point2D::default(), Point2D::new(1, 2));
    //     }

    //     #[test]
    //     fn point_addition() {
    //         let a = Point2D::new(1, 2);
    //         let b = Point2D::new(3, 4);

    //         assert_eq!(a + b, Point2D::new(4, 6));
    //     }

    //     #[test]
    //     fn point_subtraction() {
    //         let a = Point2D::new(1, 2);
    //         let b = Point2D::new(3, 4);

    //         assert_eq!(a - b, Point2D::new(-2, -2));
    //     }

    //     #[test]
    //     fn point_ordering() {
    //         let mut points: Vec<Point2D> = vec![(1, 4).into(), (0, 0).into(), (-3, -10).into()];
    //         points.sort();

    //         assert_eq!(points, vec![(0, 0).into(), (1, 4).into(), (-3, -10).into()]);
    //     }
}
