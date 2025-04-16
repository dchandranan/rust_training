/// A point in 2D space
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

/// A circle defined by center and radius
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub center: Point,
    pub radius: f64,
}

/// A triangle defined by three points
#[derive(Debug, Clone, PartialEq)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

/// A rectangle defined by two opposite corners
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

/// Enumeration of all supported shapes
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Point(Point),
    Circle(Circle),
    Triangle(Triangle),
    Rectangle(Rectangle),
}

// Point functions
/// Calculates distance between two points
///
/// # Examples
/// ```
/// use p22::figures::Point;
/// let p1 = Point { x: 0.0, y: 0.0 };
/// let p2 = Point { x: 3.0, y: 4.0 };
/// assert_eq!(p22::figures::point_distance(&p1, &p2), 5.0);
/// ```
pub fn point_distance(p1: &Point, p2: &Point) -> f64 {
    ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
}

/// Area of a point (always 0)
pub fn point_area(_: &Point) -> f64 {
    0.0
}

/// Perimeter of a point (always 0)
pub fn point_perimeter(_: &Point) -> f64 {
    0.0
}

// Circle functions
/// Area of a circle
///
/// # Examples
/// ```
/// use p22::figures::{Circle, Point};
/// let circle = Circle { center: Point { x: 0.0, y: 0.0 }, radius: 3.0 };
/// assert_eq!(p22::figures::circle_area(&circle), 28.274333882308138);
/// ```
pub fn circle_area(circle: &Circle) -> f64 {
    std::f64::consts::PI * circle.radius.powi(2)
}

/// Circumference of a circle
///
/// # Examples
/// ```
/// use p22::figures::{Circle, Point};
/// let circle = Circle { center: Point { x: 1.0, y: 1.0 }, radius: 2.5 };
/// assert_eq!(p22::figures::circle_perimeter(&circle), 15.707963267948966);
/// ```
pub fn circle_perimeter(circle: &Circle) -> f64 {
    2.0 * std::f64::consts::PI * circle.radius
}

// Triangle functions
/// Area of a triangle
///
/// # Examples
/// ```
/// use p22::figures::{Triangle, Point};
/// let triangle = Triangle {
///     a: Point { x: 0.0, y: 0.0 },
///     b: Point { x: 4.0, y: 0.0 },
///     c: Point { x: 0.0, y: 3.0 },
/// };
/// assert_eq!(p22::figures::triangle_area(&triangle), 6.0);
/// ```
pub fn triangle_area(triangle: &Triangle) -> f64 {
    let a = point_distance(&triangle.a, &triangle.b);
    let b = point_distance(&triangle.b, &triangle.c);
    let c = point_distance(&triangle.c, &triangle.a);
    let s = (a + b + c) / 2.0;
    (s * (s - a) * (s - b) * (s - c)).sqrt()
}

/// Perimeter of a triangle
///
/// # Examples
/// ```
/// use p22::figures::{Triangle, Point};
/// let triangle = Triangle {
///     a: Point { x: 0.0, y: 0.0 },
///     b: Point { x: 3.0, y: 0.0 },
///     c: Point { x: 0.0, y: 4.0 },
/// };
/// assert_eq!(p22::figures::triangle_perimeter(&triangle), 12.0);
/// ```
pub fn triangle_perimeter(triangle: &Triangle) -> f64 {
    point_distance(&triangle.a, &triangle.b)
        + point_distance(&triangle.b, &triangle.c)
        + point_distance(&triangle.c, &triangle.a)
}

// Rectangle functions
/// Area of a rectangle
///
/// # Examples
/// ```
/// use p22::figures::{Rectangle, Point};
/// let rect = Rectangle {
///     top_left: Point { x: 0.0, y: 4.0 },
///     bottom_right: Point { x: 3.0, y: 0.0 },
/// };
/// assert_eq!(p22::figures::rectangle_area(&rect), 12.0);
/// ```
pub fn rectangle_area(rect: &Rectangle) -> f64 {
    let width = (rect.bottom_right.x - rect.top_left.x).abs();
    let height = (rect.top_left.y - rect.bottom_right.y).abs();
    width * height
}

/// Perimeter of a rectangle
///  
/// # Examples
/// ```
/// use p22::figures::{Rectangle, Point};
/// let rect = Rectangle {
///     top_left: Point { x: 1.0, y: 5.0 },
///     bottom_right: Point { x: 4.0, y: 2.0 },
/// };
/// assert_eq!(p22::figures::rectangle_perimeter(&rect), 12.0);
/// ```
pub fn rectangle_perimeter(rect: &Rectangle) -> f64 {
    let width = (rect.bottom_right.x - rect.top_left.x).abs();
    let height = (rect.top_left.y - rect.bottom_right.y).abs();
    2.0 * (width + height)
}

// Shape functions
/// Area for any shape
pub fn shape_area(shape: &Shape) -> f64 {
    match shape {
        Shape::Point(p) => point_area(p),
        Shape::Circle(c) => circle_area(c),
        Shape::Triangle(t) => triangle_area(t),
        Shape::Rectangle(r) => rectangle_area(r),
    }
}

/// Perimeter for any shape
pub fn shape_perimeter(shape: &Shape) -> f64 {
    match shape {
        Shape::Point(p) => point_perimeter(p),
        Shape::Circle(c) => circle_perimeter(c),
        Shape::Triangle(t) => triangle_perimeter(t),
        Shape::Rectangle(r) => rectangle_perimeter(r),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_area() {
        let p = Point { x: 1.0, y: 2.0 };
        assert_eq!(point_area(&p), 0.0);
    }

    #[test]
    fn test_circle_area() {
        let c = Circle {
            center: Point { x: 0.0, y: 0.0 },
            radius: 2.0,
        };
        assert_eq!(circle_area(&c), std::f64::consts::PI * 4.0);
    }

    #[test]
    fn test_triangle_area() {
        let t = Triangle {
            a: Point { x: 0.0, y: 0.0 },
            b: Point { x: 4.0, y: 0.0 },
            c: Point { x: 0.0, y: 3.0 },
        };
        assert_eq!(triangle_area(&t), 6.0);
    }

    #[test]
    fn test_rectangle_area() {
        let r = Rectangle {
            top_left: Point { x: 0.0, y: 4.0 },
            bottom_right: Point { x: 3.0, y: 0.0 },
        };
        assert_eq!(rectangle_area(&r), 12.0);
    }
}
