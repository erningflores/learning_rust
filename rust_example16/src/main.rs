fn main() {
    println!("Literals, Inference, Type Aliases!\n");

    let x = 1_u8;
    let y = 2_u32;
    let z = 3_f32;

    let i = 1;
    let f = 1.0;

    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    /*
    std::mem::size_of_val() - is a full path
    std - is a crate
    mem - is a module
    size_of_val() - is a function
    */

    //It can infer the type and how it is used
    let elem = 5_u8;
    let mut vec = Vec::new();
    vec.push(elem);
    println!("{:?}", vec);

    //Type Aliases
    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;
    println!("{} nanoseconds + {} inches = {} unit?", nanoseconds, inches, nanoseconds + inches);
}
