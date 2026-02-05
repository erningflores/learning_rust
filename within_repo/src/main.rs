fn main() {
    println!("Hello, world!");

    let x: i32 = -40;
    let y: u64 = 100;
    println!("Signed integer: {}", x);
    println!("Unsigned integer: {}", y);

    let a1: char = 'A';
    println!("Letter: {}", a1);

    let b1: bool = true;
    println!("Is it snowing: {}", b1);

    //slice
    let name: &str = "Jacob";
    println!("My name is {}", name);

    let f1: f32 = 3.14;
    println!("Pi is {}", f1);
}
