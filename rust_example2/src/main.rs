//You cannot use print below either in fmt::Display or fmt::Debug. Comment it to be ignored.
//struct UnPrintable(i32);

//by using #[derive(Debug)], you can print it through fmt::Debug
#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

// 'a means a lifetime varaible. So, &'a str is a lifetime string slice variable
#[derive(Debug)]
struct Person<'a>{
    name: &'a str,
    age: u8,
}

fn main() {
    println!("fmt::Debug!");

    println!("{:?} months in a year", 12);
    println!("{1:?} {0:?} is the {actor:?} name.", "Slater", "Christian", actor="actor's");

    //hoisting does not work on struct, function does.
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));

    let name = "Peter";
    let age = 27;
    let peter = Person{name, age};
    //{:#?} this is pretty printing
    println!("{:#?}", peter);
}
