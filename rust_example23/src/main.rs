#[allow(dead_code)]
enum Color{
    Red,
    Blue,
    Green,
    //These likewise tie u32 tuples to different color names: color models.
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}

fn main() {
    println!("match!\n");

    let number = 13;
    println!("Tell me about {}", number);

    match number{
        1 => println!("One!"),
        2 | 3 | 5 | 7 | 11 => println!("This is a prime!"),
        13..19 => println!("A teen"),
        _ => println!("Ain't special"),
    }

    let boolean = true;
    let binary = match boolean{
        false => 0,
        true => 1,
    };

    println!("{} -> {}", boolean, binary);

    //a match can destructure tuples, arrays, slices, enums, pointers, structures
    //tuples
    let triple = (0, -2, 3);
    println!("Tell me about {:?}", triple);
    match triple{
        (0, y, z) => println!("First is `0`, `y` is {:?}, `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        (.., 2) => println!("Last is `2` and the rest doesn't matter"),
        (3, .., 4) => println!("First is `3`, last is `4` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }

    //arrays and slices
    let array = [1, -2, 6];
    match array{
        [0, second, third] => 
            println!("array[0] = 0, array[1] = {}, array[2] = {}", second, third),
        [1, _,  third] =>
            println!("array[0] = 1, array[2] = {} and array[1] was ignored", third),
        [-1, second, ..] =>
            println!("array[0] = -1, array[1] = {} and all the rest are ignored", second),
        [3, second, tail @ ..] =>
            println!("array[0] = 3, array[1] = {} and the other elements were {:?}", second, tail),
        [first, middle @ .., last] => 
            println!("array[0] = {}, middle = {:?}, array[2] = {}", first, middle, last),
    }

    //enums
    let color = Color::RGB(122, 17, 40);

    println!("What color is it?");
    match color{
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red: {}, Green: {}, Blue: {}", r, g, b),
        Color::HSV(h, s, v) => println!("Color: {}, Saturation: {}, Value: {}", h, s, v),
        Color::HSL(h, s, l) => 
            println!("Color: {}, Saturation: {}, Lightness: {}", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, Magenta: {}, Yellow: {}", c, m, y),
        Color::CMYK(c, m, y, k) =>
            println!("Cyan: {}, Magenta: {}, Yellow: {}, Key (Black): {}", c, m, y, k),
    }
}
