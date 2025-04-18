use std::fmt::Debug;

#[derive(Eq, PartialEq, Debug)]
pub enum Choice<T, U> {
    First(T),
    Second(U),
}

pub trait Shape {
    const NAME: &'static str;

    fn perimeter(&self) -> f32;
    fn area(&self) -> f32;
    fn scale(&mut self, factor: f32);

    fn area_to_perimeter(&self) -> f32 {
        let p = self.perimeter();
        if p == 0.0 { 0.0 } else { self.area() / p }
    }

    fn biggest_shape<'a, 'b, T: Shape>(&'a self, other: &'b T) -> Choice<&'a Self, &'b T> {
        if self.area() > other.area() {
            Choice::First(self)
        } else {
            Choice::Second(other)
        }
    }

    fn print_properties(&self) {
        println!("Name: {}", Self::NAME);
        println!("Area: {}", self.area());
        println!("Perimeter: {}", self.perimeter());
    }
}

// Concrete shape implementations
#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl Shape for Point {
    const NAME: &'static str = "Point";

    fn perimeter(&self) -> f32 {
        0.0
    }
    fn area(&self) -> f32 {
        0.0
    }
    fn scale(&mut self, _factor: f32) {}
}

#[derive(Debug)]
pub struct Triangle {
    pub a: Point,
    pub b: Point,
    pub c: Point,
}

impl Triangle {
    pub fn new(a: Point, b: Point, c: Point) -> Self {
        Self { a, b, c }
    }

    fn side_length(p1: &Point, p2: &Point) -> f32 {
        ((p2.x - p1.x).powi(2) + (p2.y - p1.y).powi(2)).sqrt()
    }
}

impl Shape for Triangle {
    const NAME: &'static str = "Triangle";

    fn perimeter(&self) -> f32 {
        Self::side_length(&self.a, &self.b)
            + Self::side_length(&self.b, &self.c)
            + Self::side_length(&self.c, &self.a)
    }

    fn area(&self) -> f32 {
        let a = Self::side_length(&self.a, &self.b);
        let b = Self::side_length(&self.b, &self.c);
        let c = Self::side_length(&self.c, &self.a);
        let s = self.perimeter() / 2.0;
        (s * (s - a) * (s - b) * (s - c)).sqrt()
    }

    fn scale(&mut self, factor: f32) {
        self.a.x *= factor;
        self.a.y *= factor;
        self.b.x *= factor;
        self.b.y *= factor;
        self.c.x *= factor;
        self.c.y *= factor;
    }
}

#[derive(Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: f32,
}

impl Circle {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Shape for Circle {
    const NAME: &'static str = "Circle";

    fn perimeter(&self) -> f32 {
        2.0 * std::f32::consts::PI * self.radius
    }

    fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius.powi(2)
    }

    fn scale(&mut self, factor: f32) {
        self.radius *= factor;
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Rectangle {
    pub fn new(top_left: Point, bottom_right: Point) -> Self {
        Self {
            top_left,
            bottom_right,
        }
    }
}

impl Shape for Rectangle {
    const NAME: &'static str = "Rectangle";

    fn perimeter(&self) -> f32 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.top_left.y - self.bottom_right.y).abs();
        2.0 * (width + height)
    }

    fn area(&self) -> f32 {
        let width = (self.bottom_right.x - self.top_left.x).abs();
        let height = (self.top_left.y - self.bottom_right.y).abs();
        width * height
    }

    fn scale(&mut self, factor: f32) {
        self.top_left.x *= factor;
        self.top_left.y *= factor;
        self.bottom_right.x *= factor;
        self.bottom_right.y *= factor;
    }
}

#[derive(Debug)]
pub enum DynamicShape {
    Point(Point),
    Triangle(Triangle),
    Circle(Circle),
    Rectangle(Rectangle),
}

impl Shape for DynamicShape {
    const NAME: &'static str = "DynamicShape";

    fn perimeter(&self) -> f32 {
        match self {
            Self::Point(p) => p.perimeter(),
            Self::Triangle(t) => t.perimeter(),
            Self::Circle(c) => c.perimeter(),
            Self::Rectangle(r) => r.perimeter(),
        }
    }

    fn area(&self) -> f32 {
        match self {
            Self::Point(p) => p.area(),
            Self::Triangle(t) => t.area(),
            Self::Circle(c) => c.area(),
            Self::Rectangle(r) => r.area(),
        }
    }

    fn scale(&mut self, factor: f32) {
        match self {
            Self::Point(p) => p.scale(factor),
            Self::Triangle(t) => t.scale(factor),
            Self::Circle(c) => c.scale(factor),
            Self::Rectangle(r) => r.scale(factor),
        }
    }
}

#[derive(Debug)]
pub enum SliceRef<'a, 'b, T: Shape, U: Shape> {
    First(&'a [T]),
    Second(&'b [U]),
}
/// Return slice with bigger perimeter to area ratio
pub fn bigger_slice<'a, 'b, T: Shape + Debug, U: Shape + Debug>(
    first: &'a [T],
    second: &'b [U],
) -> SliceRef<'a, 'b, T, U> {
    let first_ratio = first
        .iter()
        .map(|s| s.perimeter() / s.area().max(f32::EPSILON))
        .fold(0.0, f32::max);
    let second_ratio = second
        .iter()
        .map(|s| s.perimeter() / s.area().max(f32::EPSILON))
        .fold(0.0, f32::max);

    if first_ratio >= second_ratio {
        println!("{:?}", first);
        SliceRef::First(first)
    } else {
        println!("{:?}", second);
        SliceRef::Second(second)
    }
}

#[cfg(test)]
mod tests {
    use super::Shape;
    use super::*;

    #[test]
    fn test_shape_properties() {
        let mut point = Point::new(1.0, 2.0);
        assert_eq!(Point::NAME, "Point");
        assert_eq!(point.perimeter(), 0.0);
        point.scale(2.0);
        assert_eq!(point.x, 1.0);

        let circle = Circle::new(Point::new(0.0, 0.0), 1.0);
        circle.print_properties();
    }

    #[test]
    fn test_biggest_shape() {
        let point = Point::new(0.0, 0.0);
        let circle = Circle::new(Point::new(0.0, 0.0), 1.0);
        let bigger = Shape::biggest_shape(&point, &circle);
        if let Choice::Second(bigger) = bigger {
            assert_eq!(bigger.area(), circle.area());
        } else {
            panic!("This should not be reached");
        }
    }

    #[test]
    fn test_dynamic_shapes() {
        let shapes = vec![
            DynamicShape::Point(Point::new(0.0, 0.0)),
            DynamicShape::Circle(Circle::new(Point::new(0.0, 0.0), 2.0)),
        ];

        assert_eq!(shapes[0].perimeter(), 0.0);
        assert_eq!(shapes[1].area(), std::f32::consts::PI * 4.0);
    }
}
