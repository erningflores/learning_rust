#[derive(Debug)]
struct A;

#[derive(Debug)]
struct Single(A);

#[derive(Debug)]
struct SingleGen<T>(T);

fn main() {
    println!("Generic!");

    let _s = Single(A);

    let _char: SingleGen<char> = SingleGen('a');
    let _t = SingleGen(A);
    let _i32 = SingleGen(6);
    println!("_s: {:?}", _s);
    println!("_char: {:?}", _char);
    println!("_t: {:?}", _t);
    println!("_i32: {:?}", _i32);
}
