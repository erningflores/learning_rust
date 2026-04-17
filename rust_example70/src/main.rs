/*
extern crate rand;
use rand::fill;

fn random_vec() -> &'static [u64; 100]{
    let mut rng = rand::rng();
    let mut boxed = Box::new([0; 100]);

    boxed.fill(&mut rng);
    Box::leak(boxed)
}
*/

//elided_input and annotated_input are the same
fn elided_input(x: &i32){
    println!("`elided_input`: {}", x);
}

fn annotated_input<'a>(x: &'a i32){
    println!("`annotated_input`: {}", x);
}

//elided_pass and annotated pass are basically the same.
fn elided_pass(x: &i32) -> &i32{
    x
}

fn annotated_pass<'a>(x: &'a i32) -> &'a i32{
    x
}

fn main() {
    println!("Elision!\n");
    /*
    let first: &'static [u64; 100] = random_vec();
    let second: &'static [u64; 100] = random_vec();
    assert_ne!(first, second);
    */

    let x = 3;
    elided_input(&x);
    annotated_input(&x);

    println!("`elided_pass`: {}", elided_pass(&x));
    println!("`annotaed_pass`: {}", annotated_pass(&x));
}
