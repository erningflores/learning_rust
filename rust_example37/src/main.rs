//#![feature(never_type)]

//diverging function never returns
/*
fn foo() -> !{
    panic!("This will never returns.");
}
*/

//this is different from diverging funcitons
fn some_fn(){
    ()
}

fn sum_odd_numbers(up_to: u32) -> u32{
    let mut acc = 0;
    for i in 0..up_to{
        let addition: u32 = match i % 2 == 1{
            true => i,
            false => continue,
        };
        acc += addition;
    }
    acc
}

fn main() {
    println!("Diverging functions!\n");

    //foo();
    let _a: () = some_fn();
    println!("This function returns and you can see this line.");

    /*
    //error unreacheable statement indeed.
    let _x: ! = foo();
    println!("You will never see this line.");
    */

    println!("Sum of odd numbers up to 9 (excluding): {}", sum_odd_numbers(9));
}
