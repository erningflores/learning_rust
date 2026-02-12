fn main() {
    println!("Variable Bindings!\n");

    let _an_integer = 1u32;
    let _a_boolean = true;
    let _unit = ();

    let _copied_integer = _an_integer;

    //println!("An integer: {:?}", _an_integer);
    println!("Copied integer: {:?}", _copied_integer);
    println!("A boolean: {:?}", _a_boolean);
    println!("Meet the unit value: {:?}", _unit);

    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);
    //variable bindings are immutable by default.
    // you can overrirde by using mut just like the code above.
    //_immutable_binding += 1 //this will cause an error

    //within main scope block
    let long_lived_binding = 1;
    {
        let short_lived_binding = 2;
        println!("innert short: {}", short_lived_binding);
    }
    //outside the scope of the block it was declared.
    //println!("outer short: {}", short_lived_binding);
    println!("outer long: {}", long_lived_binding);

    let shadow_binding = 1;
    {
        println!("before being shadowed: {}", shadow_binding);

        let shadow_binding = "abc";
        println!("Shadowed in inner block: {}", shadow_binding);
    }

    println!("Outside inner block: {}", shadow_binding);

    let shadow_binding = 2;
    println!("Shadowed in outer block: {}", shadow_binding);

    //it is possible to declare a variable without initialization
    //make sure to initialize it before used
    let a_binding;

    {
        let x = 2;
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding);

    /*
    let another_binding;
    println!("another binding: {}", another_binding);

    another_binding = 1;
    */

    //sample of freezing a mutable, meaning it cannot be modified
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        println!("shadowing a mutable integer: {}", _mutable_integer);

        //_mutable_integer = 50; // this is an error.
        
    }

    _mutable_integer = 3;
    println!("Can change the value of mutable integer now: {}", _mutable_integer);
}
