fn apply<F>(f: F) where
    F: FnOnce(){
        f()
    }

fn apply_to_3<F>(f: F) -> i32 where
    F: Fn(i32) -> i32{
        f(3)
    }

fn main() {
    println!("function which takes closure as an argument!\n");

    use std::mem;
    let greeting = "hello";
    //a non copy; to_owned() creates own data from a copy one.
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        //greeting is by reference requires fn
        println!("I said {}.", greeting);
        //mutation forces farewell to be captured by mutable reference
        //now requires FnMut.
        farewell.push_str("!!!");
        println!("Then I screamed {}.", farewell);
        println!("Now I can sleep.zzzzz");
        //manually calling drop forces farewell to be captured by value.
        //Now requires FnOnce.
        mem::drop(farewell);
    };
    //call the function which applies the closure.
    apply(diary);

    let double = |x| 2 * x;
    println!("3 doubled: {}", apply_to_3(double));

}
