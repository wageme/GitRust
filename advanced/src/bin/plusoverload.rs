use std::ops::Add;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul for Point {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }

}

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    assert_eq!(
        Point { x: 1, y: 0 } * Point { x: 2, y: 3 },
        Point { x: 2, y: 0 }
    );

    println!("All good!");


    type Kilometers = i32;

    let x: i32 = 5;
    let y: Kilometers = 5;

    println!("x + y = {}", x+y);

    type Thunk = (i32, String);
    let f: Thunk = (2, "Bruh".to_string());

    println!("{:?}", f);

    println!("The answer is: {}", do_twice(add_one, 5));

    let mut functions: Vec<fn(String) -> i32> = vec![say_something];
    functions[0]("hmm".to_string());

    

}


fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn say_something(s: String) -> i32 {
    println!("{s}");
    1
}