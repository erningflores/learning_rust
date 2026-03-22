use std::fmt::Debug;

trait HashArea{
    fn area(&self) -> f64;
}

impl HashArea for Rectangle{
    fn area(&self) -> f64{
        self.length * self.height
    }
}

#[derive(Debug)]
struct Rectangle{
    length: f64,
    height: f64,
}

#[allow(dead_code)]
struct Triangle{
    length: f64,
    height: f64,
}

fn print_debug<T: Debug>(t: &T){
    println!("{:?}", t);
}

fn area<T: HashArea>(t: &T) -> f64{
    t.area()
}

//empty bounds
struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red{}
trait Blue{}

impl Red for Cardinal{}
impl Blue for BlueJay{}

fn red<T: Red>(_: &T) -> &'static str {
    "red"
}
fn blue<T: Blue>(_: &T) -> &'static str{
    "blue"
}

fn main() {
    println!("Bound generics\n");

    let rectangle = Rectangle{ length: 3.0, height: 4.0 };
    let _triangle = Triangle{ length: 3.0, height: 4.0 };
    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    /*
    //error since it will look for HashArea
    print_debug(&_triangle);
    println!("Area: {}", area(&_triangle));
    */

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey = Turkey;

    println!("A Cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));

    //An error since it is not bound in red
    //println!("A turkey is {}", red(&_turkey));
}
