struct Sheep{}
struct Cow{}

trait Animal{
    fn noise(&self) -> &'static str;
}

impl Animal for Sheep{
    fn noise(&self) -> &'static str{
        "baaaaaah!"
    }
}

impl Animal for Cow{
    fn noise(&self) -> &'static str{
        "mooooo!"
    }
}

fn random_animal(random_number: f64) -> Box<dyn Animal>{
    if random_number < 0.5{
        Box::new(Sheep{})
    }else {
        Box::new(Cow{})
    }
}

fn main() {
    println!("Returning traits with dyn!\n");

    let random_number = 0.234;
    let animal = random_animal(random_number);
    
    println!("You've randomly choosen an animal, it says {}", animal.noise());
}
