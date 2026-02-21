struct Foo{
    x: (u32, u32),
    y: u32,
}

struct Bar{
    foo: Foo,
}

fn main() {
    println!("match destructing pointers, struct!\n");

    //deferencing uses *
    //destructuring uses &, ref, ref mut
    
    //& here signifies a reference
    let reference = &4;
    match reference{
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    //to avoid the '&', you dereference before matching
    match *reference{
        val => println!("Got a value via dereferencing: {:?}", val),
    }

    //without using an &, means not a reference
    let _not_a_reference = 3;

    //ref means that it modifies the assignment to a reference
    let ref _is_a_reference = 3;

    let value = 5;
    let mut mut_value = 6;

    match value{
        ref r => println!("Got a reference to a value: {:?}", r),
    }

    match mut_value{
        ref mut m => {
            //the m is a reference, so you need to dereference it before you can add into it
            *m += 10;
            println!("We added 10. `mut_value`: {:?}", m);
        },
    }

    //using struct
    let foo = Foo{x: (1, 2), y: 3};
    match foo{
        Foo{x: (1, b), y} => println!("First of x is 1, b = {}, y = {}", b, y),
        Foo{y: 2, x: i} => println!("y is 2, x = {:?}", i),
        Foo{y, ..} => println!("y = {}, we don't care about x", y),
    }

    //you don't need match block to destruct a struct
    let faa = Foo{x: (3, 4), y: 5};
    let Foo{x: x0, y: y0} = faa;
    println!("Outside: x0 = {x0:?}, y0 = {y0}");

    let bar = Bar{foo: faa};
    let Bar{ foo: Foo {x: nested_x, y: nested_y}} = bar;
    println!("Nested: nested_x = {nested_x:?}, nested_y = {nested_y:?}");
}
