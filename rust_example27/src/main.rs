fn fizzbuzz_to(n: u32){
    for n in 1..=n{
        fizzbuzz(n);
    }
}

fn fizzbuzz(n: u32) -> (){
    if is_divisible_by(n, 15){
        println!("fizzbuzz");
    }else if is_divisible_by(n, 3){
        println!("fizz");
    }else if is_divisible_by(n, 5){
        println!("buzz");
    }else {
        println!("{}", n);
    }
}

fn is_divisible_by(lhs: u32, rhs: u32) -> bool{
    //return is necessary, since it is an early return
    if rhs == 0{
        return false;
    }
    //return is not necessary in here
    lhs % rhs == 0
}

fn main() {
    println!("functions!\n");

    fizzbuzz_to(100);
}
