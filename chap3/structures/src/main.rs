#![allow(dead_code)]
use std::fmt;
#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32,f32);

#[derive(Debug)]
struct Point{
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    fn area(&self) -> f32 {
        // destructure tetangle into points into x1/2 and y1/2 
        let Rectangle {
            top_left: Point {x: x1, y: y1},
            bottom_right: Point {x: x2, y: y2},
        } = *self;
        (x2 - x1).abs() * (y2 - y1).abs()
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn square(point: Point, width: f32) -> Rectangle {
    Rectangle {
        top_left: Point{x: point.x, y: point.y},
        bottom_right: Point {
            x: point.x + width,
            y: point.y + width,
        }
    }
}
fn main() {
    let name =String::from("Peter");
    let age = 27;
    let peter = Person {name, age};
    println!("{:?}", peter);
    let point: Point = Point {x: 10.3, y: 0.4};
    println!("point coordinates: {}", point);
// use point as update struct for the new point ..point
    let bottom_right = Point {x: 5.2, ..point};
    println!("second point: {}", bottom_right);
    // destructure point using let pat = expr;
    let Point  { x: left_edge, y: top_edge } = point;
    println!("left_edge: {}, top_edge: {}", left_edge, top_edge);
    let bottom_right = Point {x: 5.2, y: 0.8};
    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge}, // instantiate on the fly
        bottom_right: bottom_right,
    };
    let _unit = Unit; // instantiate a unit staruct 
    let pair = Pair(1, 0.1); // instantiate a pair struct 
    println!("pair contains {:?} and {:?}", pair.0, pair.1);
    let Pair(integer, decimal) = pair; // destructure a pair tuple struct 
    println!("pair contains {:?} and {:?}", integer, decimal);
    println!("area of rectangle: {}", _rectangle.area());
    println!("square: {:?}", square(point, 5.0));
}
