use std::fmt;

#[derive(Debug)]
struct Person{
    name: String,
    age: u8,
}

#[derive(Debug)]
struct Unit;

#[derive(Debug)]
struct Pair(i32, f32);

#[derive(Debug)]
struct Point{
    x: f32,
    y: f32,
}

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Rectangle{
    top_left: Point,
    bottom_right: Point,
}

impl fmt::Display for Rectangle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.top_left, self.bottom_right)
    }
}

fn rect_area(rect: Rectangle) -> f32{
    let Point {x: x1, y:y1} = rect.top_left;
    let Point {x: x2, y: y2} = rect.bottom_right;
    (x2 - x1) * (y2 - y1)
}

#[derive(Debug)]
struct Square{
    x: Point,
    y: Point,
}
fn square_to_rectangle(square: Square) -> Rectangle{    
    let Point {x: x1, y:y1} = square.x;
    let Point {x: x2, y: y2} = square.y;
    println!("width: {:?}", x2-x1);
    println!("height: {:?}", y2-y1);
    println!{"Area: {:?}", (x2-x1) * (y2-y1)};
    Rectangle{
        top_left: Point{x: x1, y: y1},
        bottom_right: Point{x: x2, y: y2},
    }
}

fn main() {
    println!("Struct!\n");

    let name = String::from("Peter");
    let age = 27;
    let peter = Person{ name, age };
    println!("{:?}", peter);

    let _point: Point = Point{ x: 5.2, y: 0.4 };
    let _another_point: Point = Point { x: 10.3, y: 1.2 };
    println!("point coordinates: ({}, {})", _point.x, _point.y);

    let bottom_right = Point {x: 10.3, .._another_point };
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    let Point{ x: left_edge, y: top_edge } = _point;
    let _rectangle = Rectangle {
        top_left: Point {x: left_edge, y: top_edge},
        bottom_right: bottom_right,
    };
    println!("rectangle: {:?}", _rectangle);

    let _unit = Unit;
    let _pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", _pair.0, _pair.1);

    let Pair(integer, deimal) = _pair;
    println!("pair contains: {:?} and {:?}", integer, deimal);

    println!("Area of rectangle: {}", rect_area(_rectangle));

    let _point_1 = Point{x: 2.0, y: 1.3};
    let _point_2 = Point{x: 6.1, y: 8.6};
    let _square = Square{x: _point_1, y: _point_2};
    println!("Rectangle: {:?}", square_to_rectangle(_square));
}
