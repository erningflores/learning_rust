fn drink(beverage: &str){
    if beverage == "lemonade"{
        panic!("AAAaaaa!!!!");
    }

    println!("Some refreshing {} is all I need.", beverage);
}

fn main() {
    println!("Panic!\n");

    drink("water");
    drink("lemonade");
    //below won't run, because of panic
    drink("still water");
}
