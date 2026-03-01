fn apply<F>(f: F) where
    F: Fn(){
        f();
    }

//input function
fn call_me<F: Fn()>(f: F){
    f();
}

fn function(){
    println!("I'm a function!");
}

fn main() {
    println!("type anonymity!\n");

    let x = 7;
    let print = || println!("{}", x);
    apply(print);

    let closure = || println!("I'm a closure!");
    call_me(closure);
    call_me(function);
}
