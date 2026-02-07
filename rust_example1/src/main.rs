fn main() {
    println!("{} days", 31);
    //positional
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");
    //named arguments
    println!("{subject} {verb} {object}", object=" over the lazy dog", subject="the quick brown fox", verb="jump");

    //different formatting
    println!("Base 10: {}", 69420);
    println!("Base 2 (binary): {:b}", 69420);
    println!("Base 8 (octal): {:o}", 69420);
    println!("Base 16 (hexadecimal): {:x}", 69420);

    //right justify width
    println!("{number:>5}", number=1);
    //padding with zeros to the right
    println!("{number:0>5}", number=1);
    //padding with zeros to the left
    println!("{number:0<5}", number=1);

    //named arguments. take note of the $ sign
    println!("{number:0>width$}", number=1, width=5);

    //rust checks the number of arguments. it will result an error below
    //println!("My name is {0}, {1} {0}", "Bond");

    /*
    //allowing the dead code will make it run even there is a warning.
    #[allow(dead_code)]
    struct Structure(i32);
    //the print below won't print since it does not implement fmt::display
    println!("This struct '{}' won't print...", Structure(3));
    */

    //rust allows to capture an argument from a variable
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");

    let pi = 3.141592;
    println!("Pi: {:.3}", pi);
}
