fn main() {
    println!("Partial moves!\n");

    #[derive(Debug)]
    struct Person{
        name: String,
        age: Box<u8>,
    }

    let person = Person{
        name: String::from("Alice"),
        age: Box::new(20),
    };

    let Person{name, ref age} = person;
    println!("The person's age is: {}", age);
    println!("The person's name is: {}", name);

    //error below since person's name is moved
    //println!("The person's struct is: {:?}", person);

    println!("The person's age from the person struct: {}", person.age);
}
