#![allow(unreachable_code, unused_labels)]

fn main() {
    println!("Nesting Loop, labels, return!\n");

    //loop
    'outer: loop{
        println!("Entered the outer loop");
        'inner: loop{
            println!("Entered the inner loop");
            //note: if you use break here without label,
            //it will only break the inner loop
            break 'outer;
        }
        println!("This point will never be reached");
    }

    println!("Exited the outer loop");

    let mut counter = 0;

    let result = loop{
        counter += 1;
        if counter == 10{
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
    println!("Result: {}", result);

    //while loop
    let mut n = 1;

    while n < 101{
        if n % 15 == 0{
            println!("fizzbuzz");
        }else if n % 3 == 0{
            println!("fizz");
        }else if n % 5 == 0{
            println!("buzz");
        }else {
            println!("{}", n);
        }
        n += 1;
    }

}
