fn main() {
    let age: u16 = 18;
    if age >= 18 {
        println!("You can drive a car!");
    }else {
        println!("You can't drive a car.");
    }

    let number = 6;
    if number % 4 == 0 {
        println!("Number is divisible by 4");
    }else if number % 3 == 0 {
        println!("Number is divisible by 3");
    }else if number % 2 == 0 {
        println!("Number is divisible by 2");
    }else {
        println!("Number is not divisible by 4, 3, or 2..");
    }

    let condition = true;
    let x1 = if condition {5} else {6};
    println!("Number: {x1}");

    //using loop
    let mut counter = 0;
    let result = loop{
        counter += 1;

        println!("Counter: {}", counter);
        if counter >= 5 {
            break counter * 2;
        }
    };
    println!("Result: {}", result);

    //another loop example
    let mut count = 0;
    'counting_up: loop{
        println!("Count: {count}");

        let mut remaining = 10;
        loop{
            println!("Remaining: {remaining}");

            if remaining == 9 {
                break;
            }
            if count == 2{
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    //while loop
    let mut timer = 3;
    while timer != 0 {
        println!("{timer}");
        timer -= 1;
    }
    println!("Boom!!!");

    //for loop
    let a = [1, 2, 3, 4, 5, 6, 7, 8];
    let b = ["a", "b", "c", "d", "e", "f", "g", "h"];
    
    for element in a {
        println!("{element}");
    }

    for element in b {
        println!("{element}");
    }
}
