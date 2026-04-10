fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32){
    println!("x is {} and y is {}", x, y);
}

/*
fn failed_borrow<'a>(){
    let _x = 12;
    let _y: &'a i32 = &_x;

}
*/

fn print_one<'a>(x: &'a i32){
    println!("`print one`: x is {}", x);
}

fn add_one<'a>(x: &'a mut i32){
    *x += 1;
}

fn print_multi<'a, 'b>(x: &'a i32, y: &'b i32){
    println!("`print multi`: x is {}, y is {}", x, y);
}

fn pass_x<'a, 'b>(_x: &'a i32, _y: &'b i32) -> &'a i32{
    _x
}


fn main() {
    println!("Lifetime elision!\n");
    let (four, nine) = (4, 9);
    print_refs(&four, &nine);
    //failed_borrow();

    let x = 7;
    let y = 9;

    print_one(&x);
    print_multi(&x, &y);

    let z = pass_x(&x, &y);
    print_one(z);

    let mut t = 3;
    add_one(&mut t);
    print_one(&t);
}
