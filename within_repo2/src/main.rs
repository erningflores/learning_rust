fn main() {
    let numbers: [i32; 5] = [1,2,3,4,5];
    println!("Numbers: {:?}", numbers);

    let fruits: [&str; 3] = ["apple", "banana", "orange"];
    println!("Fruits: {:?}", fruits);
    println!("Fruit 1: {}", fruits[0]);
    println!("Fruit 2: {}", fruits[1]);
    println!("Fruit 3: {}", fruits[2]);

    //tuple
    let human: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human tuple: {:?}", human);

    let my_mix_tuple = ("Kratos", 55, true, [1,2,3,4,5]);
    println!("My mix tuple: {:?}", my_mix_tuple);

    //slice
    let number_slices:&[i32] = &[1,2,3,4,5];
    println!("Number slices: {:?}", number_slices);

    let animal_slice: &[&str] = &["lion", "elephant", "crocodile"];
    println!("animal reference string slice: {:?}", animal_slice);

    let book_slices: &[String] = &["IT".to_string(), "Harry Potter".to_string(), "Zen".to_string()];
    println!("book string slice: {:?}", book_slices);

    //String - is mutable, owned, heap memory (slow)
    //all data types are immutable by default. just add mut to the declaration
    let mut stone_cold: String = String::from("Hell, ");
    stone_cold.push_str("Yeah!");
    println!("Stone Cold says, {}", stone_cold);

    //&str - string slice (reference to a string, now owned, stack, immutable, faster)
    let hello_world: String = String::from("Hello, World!");
    let string_slice: &str = &hello_world[0..5];
    println!("Slice value: {}", string_slice);
}
