#[derive(Debug)]
enum Fruit{
    Apple, Orange, Banana, Kiwi, Lemon,
}

fn main() {
    println!("get_or_insert_with\n");

    let mut my_fruit: Option<Fruit> = None;
    
    let get_lemon_as_fallback = ||{
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };

    let first_available_fruit = my_fruit.get_or_insert_with(get_lemon_as_fallback);
    println!("first_available_fruit is: {:?}", first_available_fruit);
    println!("my_fruit is: {:?}", my_fruit);
}
