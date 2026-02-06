fn main() {
    println!("Using Enum");

    let go = Direction::North;
    display_direction(go);

    let message = Message::Write(String::from("Hello Rust"));
    process_message(message);

    //Option<T>
    let result: Option<f64> = divide_option(10.0, 2.0);
    match result {
        Option::Some(x) => println!("Result1: {}", x),
        Option::None => println!("Cannot divide by zero!"),
    }

    //Result<T, E>
    match divide_result(100.25, 5.0){
        Result::Ok(x) => println!("Result2: {}", x),
        Result::Err(err) => println!("Error: {}", err),
    }
}

enum Direction{
    North,
    South,
    East,
    West,
}

fn display_direction(direction: Direction){
    match direction {
        Direction::North => println!("Heading North!"),
        Direction::South => println!("Heading South!"),
        _ => println!("Heading elsewhere.")
    }
}

enum Message{
    Quit,
    Move{x: i32, y: i32},       //name fields like a struct
    Write(String),              //Tuple data (a string)
    ChangeColor(i32, i32, i32), //Tuple data (rgb values)
}

fn process_message(msg: Message){
    match msg{
        Message::Quit => {
            println!("Quiting");
        }
        Message::Move{x, y} => {
            println!("Move to x: {}, y: {}", x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Color change R: {}, G: {}, B: {}", r, g, b);
        }
    }
}

//Opion<T>
enum Option<T>{
    Some(T),
    None,
}

fn divide_option(numerator: f64, denominator: f64) -> Option<f64>{
    if denominator == 0.0 {
        Option::None
    }else {
        Option::Some(numerator/denominator)
    }
}

//Result<T, E>
enum Result<T, E>{
    Ok(T),
    Err(E),
}

fn divide_result(numerator: f64, denominator: f64) -> Result<f64, String>{
    if denominator == 0.0 {
        Result::Err("Cannot divide by Zero".to_string())
    }else {
        Result::Ok(numerator/denominator)
    }
}

