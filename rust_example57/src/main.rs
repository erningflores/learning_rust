fn main() {
    println!("Mutability!\n");

    let immutable_box = Box::new(5u32);

    println!("immutable_box contains: {}", immutable_box);

    //you change ownership in this stage, and mutability can be changed
    let mut mutable_box = immutable_box;
    
    println!("mutable_box contains: {}", mutable_box);
    *mutable_box = 4;
    println!("mutable_box now contains: {}", mutable_box);
}
