//supress all errors from casts which overflow
#![allow(overflowing_literals)]

fn main() {
    println!("Casting Types!\n");

    let decimal = 65.4321_f32;

    //not allowed, no implicit conversion
    //let integer: u8 = decimal; //commented out

    //this is allowed an explicit conversion
    let integer = decimal as u8;
    let character = integer as char;

    //limitation of explicit conversion
    //example float into decimal, not possible
    //let character = decimal as char; //commented out

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 as a u16 is: {}", 1000 as u16);
    println!("1000 as a u8 is: {}", 1000 as u8);

    println!("  -1 as a u8: {}", (-1i8) as u8);
    println!("1000 mod 256 is: {}", 1000 % 256);
    println!("  128 as a i16 is: {}", 128 as i16);
    println!("  128 as a i8 is: {}", 128 as i8);

    println!("1000 as u8 is: {}", 1000 as u8);
    println!("232 as i8 is: {}", 232 as i8);

    //special case
    println!("300.0 as u8 is: {}", 300.0_f32 as u8);
    println!("-100 as u8 is: {}", -100.0_f32 as u8);
    println!("  nan as u8 is: {}", f32::NAN as u8);

    //to avoid this special case use unsafe methods but too dangerous
    unsafe{
        println!("300.0 as u8 is: {}", 300.0_f32.to_int_unchecked::<u8>());
        println!("-100 as u8 is: {}", (-100.0_f32).to_int_unchecked::<u8>());
        println!("  nan as u8 is: {}", f32::NAN.to_int_unchecked::<u8>());
    }

}
