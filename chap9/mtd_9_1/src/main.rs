// Allow dead code

#![allow(dead_code)]

// Define a struct Point with two fields x and y of type f64.
#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

// Implement a method origin() that returns a Point with x and y set to 0.0.
impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x, y }
    }
}

use std::fmt;
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// Define rectangle struct based on Point struct
#[derive(Debug)]
struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}
// implment Display trait for rectangle
impl fmt::Display for Rectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.p1, self.p2)
    }
}
// Define PAir tuple which allocates two integer on the heap
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn new(a: i32, b: i32) -> Pair {
        Pair(Box::new(a), Box::new(b))
    }
    fn destroy(self) {
        // the Pair gets destroyed here by destructing it into its parts which than go out of scope
        // taking advantage of the Drop trait to deallocate the heap memory
        // destructure self
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
        // first and second get dropped here
    }
}

fn main() {
    let p1 = Point::new(1_f64, 2_f64);
    println!("Point p1: {}", p1);
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };
    println!("Rectangle: {}", rectangle);
    println!("Area: {}", rectangle.area());
    println!("Perimeter: {}", rectangle.perimeter());
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    // rectangle.translate(1.0, 1.0); // error: cannot borrow immutable local variable `rectangle` as mutable
    square.translate(1.0, 1.0);
    let pair = Pair::new(1, 2);
    pair.destroy();
    // pair.destroy(); // error: use of moved value: `pair
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_origin() {
        let p = Point::origin();
        assert_eq!(p.x, 0.0);
        assert_eq!(p.y, 0.0);
    }

    #[test]
    fn test_new() {
        let p = Point::new(1.0, 2.0);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
    }
    #[test]
    fn test_area() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let r = Rectangle { p1, p2 };
        assert_eq!(r.area(), 4.0);
    }

    #[test]
    fn test_perimeter() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let r = Rectangle { p1, p2 };
        assert_eq!(r.perimeter(), 8.0);
    }
    #[test]
    fn test_translate() {
        let p1 = Point::new(1.0, 2.0);
        let p2 = Point::new(3.0, 4.0);
        let mut r = Rectangle { p1, p2 };
        r.translate(1.0, 1.0);
        assert_eq!(r.p1.x, 2.0);
        assert_eq!(r.p1.y, 3.0);
        assert_eq!(r.p2.x, 4.0);
        assert_eq!(r.p2.y, 5.0);
    }
    #[test]
    fn test_pair() {
        let pair = Pair::new(1, 2);
        pair.destroy();
    }
}
