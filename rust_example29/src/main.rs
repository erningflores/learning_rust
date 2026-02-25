fn main() {
    println!("Closures!");

    let outer_var = 42;
    
    let closure_anotated = |i: i32| -> i32{i + outer_var};
    let closure_inferred = |i|          i + outer_var;
    println!("closure anotated: {}", closure_anotated(1));
    println!("closure inferred: {}", closure_inferred(1));
    
    let one = || 1;
    println!("closure returning one: {}", one());
}
