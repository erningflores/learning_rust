use std::mem;

fn main() {
    println!("(str::mem)!\n");

    //swap
    let mut x = 5;
    let mut y = 32;
    println!("x is: {} and y is: {}", x, y);
    mem::swap(&mut x, &mut y);
    println!("x is now: {} and y is now: {}", x, y);

    //replace
    let mut v = Vec::new();
    v.push(1);
   
    let old_v = mem::replace(&mut v, Vec::new());
    println!("v: {:?}", v);
    println!("old_v: {:?}", old_v);

    //take
    let mut s = String::from("hello");
    let taken_s = mem::take(&mut s);
    println!("s is now: {}", s);
    println!("taken_s is now: {}", taken_s);
    //size_of() and align_of()
}
