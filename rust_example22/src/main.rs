fn main() {
    println!("for, range, iterators");

    //range a..b where a inclusive and b exclusive
    let range = 1..10;
    println!("Range: {:?}", range);

    for i in range{
        println!("range: {}", i);
    }

    //a..b, if you want to include b, just add =
    for n in 1..=100{
        if n % 15 == 0{
            println!("fizzbuzz");
        }else if n % 3 == 0{
            println!("fizz");
        }else if n % 5 == 0{
            println!("buzz");
        }else {
            println!("{}", n);
        }
    }

    //converting collections into iterators
    //into_iter, iter, iter_mut
    let names = vec!["Bob", "Frank", "Ferris"];

    //iter borrows each element in the collection
    for name in names.iter(){
        match name{
            &"Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    println!("names: {:?}", names);

    //into_iter consumes the collection. Exact data is provided in each iteration
    for name in names.into_iter(){
        match name{
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }

    //println!("names: {:?}", names);

    //iter_mut borrow each element in the collection to be modified
    let mut animals = vec!["Lion", "Tiger", "Hippotamus"];
    for name in animals.iter_mut(){
        *name = match *name{
            "Hippotamus" => "There is an animal among us!",
            _ => "Hello",
        }
    }

    println!("animals: {:?}", animals);
}
