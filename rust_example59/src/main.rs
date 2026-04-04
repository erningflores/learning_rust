//this function takes the ownership of the box and destroys it
fn eat_box_i32(boxed_i32: Box<i32>){
    println!("Destroying box that contains: {}", boxed_i32);
}

//This function borrows an i32
fn borrow_i32(borrowed_i32: &i32){
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    println!("Borrowing!\n");

    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let _ref_to_i32: &i32 = &boxed_i32;
        
        //need to comment this line since an error occured.
        //you want to destroy what you want to borrow later
        //eat_box_i32(boxed_i32);
        
        borrow_i32(_ref_to_i32);
    }

    eat_box_i32(boxed_i32);
}
