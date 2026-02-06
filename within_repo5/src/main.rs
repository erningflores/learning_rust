fn main() {
    let x: i32 = 50;
    println!("Value of x: {}", x);
    //all variables are immutable. add mut in the declaration so you can change its value

    let mut y: i32 = 100;
    println!("Value of y: {}", y);
    y = 61;
    println!("Value of y: {}", y);

    //when declaring constant, you cannot declare it with mute.
    //it should be capitalize and the data type is required.
    const Z: i32 = 500;
    println!("Value of Z: {}", Z);

    // shadowing allows you to declare the same name variable as many as you can
    // shadowing is different from mutable since all the shadowed variable declared is immutable.
    let m: i32 = 5; // m = 5
    let m = m + 1; // m = 6
    {
        let m = m * 2; // m = 12
        println!("Value of inner m: {}", m);
    }
    println!("Value of outer m: {}", m);

    //power of shadowing is in the example below
    //a string type becomes a number with the same variable name.
    let spaces = "          ";
    let spaces = spaces.len();
    println!("Value of spaces: {}", spaces);

    // - this is a one line comment
    /*
        this is a block comment
    */ 
    display_hello();
    display_greet();
}

/// this is doc comment to print hello world
fn display_hello(){
    println!("Hello, Rust!");
}
/** 
 this is a block doc comment
*/
fn display_greet(){
    println!("Good day, Sir!");
}
