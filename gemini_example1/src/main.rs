fn main() {
    println!("Trait");

    let dog = Dog;
    let human = Human;

    make_it_talk(dog);
    make_it_talk(human);
}

trait Speak{
    fn say_hello(&self) -> String;
}

struct Dog;
impl Speak for Dog{
    fn say_hello(&self) -> String{
        "Woof!".to_string()
    }
}

struct Human;
impl Speak for Human{
    fn say_hello(&self) -> String{
        "Hello".to_string()
    }
}

fn make_it_talk<T: Speak>(creature: T){
    println!("{}", creature.say_hello());
}