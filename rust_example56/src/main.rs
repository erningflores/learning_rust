fn destroy_box(c: Box<i32>){
    println!("Destroying a box that contains {}", c);
}

fn main() {
    println!("Ownership and moves!\n");

    //stack allocated integer
    let x = 5u32;
    //copy x into y - no resources are moved
    let y = x;
    //both values can be independently used
    println!("x is {}, and y is {}", x, y);
    
    //'a' is a pointer to a heap allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    //the pointer address of 'a' is copied into 'b'
    //both are now pointers to the same heap allocated data
    //but 'b' now owns it and 'a' can no longer accesss the data
    let b = a;

    //this function takes the ownership of the heap allocated memory from 'b'
    //'b' can no longer access data after the destoy_box function
    destroy_box(b);
}
