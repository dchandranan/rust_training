use p22::figures::*;

#[test]
fn test_point_perimeter() {
    let p = Point { x: 1.0, y: 2.0 };
    assert_eq!(point_perimeter(&p), 0.0);
}

#[test]
fn test_circle_perimeter() {
    let c = Circle {
        center: Point { x: 0.0, y: 0.0 },
        radius: 3.0,
    };
    assert_eq!(circle_perimeter(&c), 6.0 * std::f64::consts::PI);
}

#[test]
fn test_triangle_perimeter() {
    let t = Triangle {
        a: Point { x: 0.0, y: 0.0 },
        b: Point { x: 3.0, y: 0.0 },
        c: Point { x: 0.0, y: 4.0 },
    };
    assert_eq!(triangle_perimeter(&t), 12.0);
}

#[test]
fn test_rectangle_perimeter() {
    let r = Rectangle {
        top_left: Point { x: 0.0, y: 5.0 },
        bottom_right: Point { x: 4.0, y: 0.0 },
    };
    assert_eq!(rectangle_perimeter(&r), 18.0);
}
