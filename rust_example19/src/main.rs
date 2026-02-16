use std::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Circle{
    radius: i32,
}

impl fmt::Display for Circle{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Circle of radius {}", self.radius)
    }
}

//using FromStr trait
impl FromStr for Circle{
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s.trim().parse(){
            Ok(num) => Ok(Circle{ radius: num }),
            Err(e) => Err(e),
        }
    }
}

fn main() {
    println!("To and From String!\n");

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    //parsing a string to a number
    //for type inferecce
    let parsed: i32 = "5".parse().unwrap();
    //using turbofish syntax
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    //FromStr
    let radius = "    3 ";
    let circle2: Circle = radius.parse().unwrap();
    println!("{}", circle2); 
}
