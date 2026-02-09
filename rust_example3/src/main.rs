//import fmt via use
use std::fmt;

//tuple struct
//use struct to implement the format display
struct Structure(i32);

//to use {}, the trait of fmt::Display must be implemented manually
impl fmt::Display for Structure{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}

//second example
#[derive(Debug)]
struct MinMax(i64, i64);

impl fmt::Display for MinMax{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
struct Point2D{
    x: f64,
    y: f64,
}

impl fmt::Display for Point2D{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

//Activity
#[derive(Debug)]
struct Complex{
    real: f64,
    imag: f64,
}

impl fmt::Display for Complex{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "{} {:+}i", self.real, self.imag)
    }
}

fn main() {
    println!("standard format display!");

    let s1 = Structure(70);
    println!("{}", s1);

    //second example
    let minmax = MinMax(0, 14);
    println!("Compare Structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range = MinMax(-300, 300);
    let small_range = MinMax(-3, 3);
    println!("The big range is {big} and the small range is {small}", small = small_range, big = big_range);

    let point = Point2D{x: 3.3, y: 7.2};
    println!("Compare Points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    //activity
    let complex1 = Complex{real: 3.3, imag: 7.2};
    let complex2 = Complex{real: 4.7, imag: -2.3};

    println!("Display Complex1: {}", complex1);
    println!("Debug Complex1: {:?}", complex1);

    println!("Display Complex2: {}", complex2);
    println!("Debug Complex2: {:?}", complex2);

}
