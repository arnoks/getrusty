use std::fmt;

struct Unprintable(i32);

#[derive(Debug)]
struct Printable(i32);

impl fmt::Display for Printable{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}
impl fmt::Display for Unprintable{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

struct MinMax(i64, i64);
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f,"({},{})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    println!(
        "Unprintable: {} Printable: {}",
        Unprintable(10),
        Printable(20));

    let mm = MinMax(3,7);
    println!("{}", mm);

    let p1 = Point2D{x: 3.0, y: 5.0};
    println!("{:?}", p1);
    println!("{}", p1);
}
