#[allow(dead_code)]
enum Temperature{
    Celcius(i32),
    Fahrenheit(i32),
}

fn age() -> u32{
    15
}

fn some_number() -> Option<u32>{
    Some(42)
}

fn main() {
    println!("match guard and binding\n");

    let temperature = Temperature::Celcius(35);

    match temperature{
        //this if t > 30 is a guard condition in this match case
        Temperature::Celcius(t) if t > 30 => println!("{}°C is above 30 Celcius", t),
        Temperature::Celcius(t) => println!("{}°C is equal to or below 39 Celcius", t),
        Temperature::Fahrenheit(t) if t > 86 => println!("{}°F is above 86 Fahrenheit", t),
        Temperature::Fahrenheit(t) => println!("{}°F is equal to or below 86 Farenheit", t), 
    }

    let number: u8 = 4;

    match number{
        i if i == 0 => println!("Zero"),
        i if i > 0 => println!("Greater than zero"),
        _ => unreachable!("Should never happen.")
    }

    //match provides @ sigil for binding values to names
    println!("Tell me what type of person you are.");

    match age(){
        0 => println!("I haven't celebrated my first birthday yet."),
        n @ 1..=12 => println!("I'm a child of age {:?}", n),
        n @ 13..=19 => println!("I'm a teen of age {:?}", n),
        n @ (20 | 21 | 22 | 23) => println!("I'm a teen of age {:?}", n),
        n => println!("I'm an old person of age {:?}", n),
    }

    match some_number(){
        Some(n @ 42) => println!("The answer: {:?}", n),
        Some(n) => println!("Not interesting... {}", n),
        _ => (),
    }
}
