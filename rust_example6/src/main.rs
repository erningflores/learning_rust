fn main() {
    println!("Variables can be declared manually(annotated type) or by inference!\n");

    //we anotate or specify the type in here
    let logical: bool = true;
    let a_float: f64 = 1.0; //this is a regular annotation
    let an_integer = 5i32; // this is a suffix annotation

    println!{"logical: {}, a_float: {}, an_integer: {}", logical, a_float, an_integer};

    //infered type or automatically detected
    let default_float = 3.0;
    let default_integer = 7;

    let mut inferred_type = 12;
    inferred_type = 4294967296i64; //i64 was annotate here instead on top decalaration

    println!("default_float: {}, default_integer: {}, inferred_type: {}", std::any::type_name_of_val(&default_float), std::any::type_name_of_val(&default_integer), std::any::type_name_of_val(&inferred_type));

    //shadowing - variables can be overwritten
    let mut mutable = 12;
    mutable = 21; // this changed is allowed since it is of the same type;
    
    /*
    //this is not allowed since the type is different from what was declared
    //need to comment it to avoid error
    mutable = true;
    */
    let mutable = true; // this was allowed through shadowing
    println!("mutable: {}", mutable);

    let my_array: [i32;5] = [1, 2, 3, 4, 5];
    let my_tuple = (5u32, 1u8, true, -5.04f32);
    
    println!("my array: {:?}", my_array);
    println!("my tuple: {:?}", my_tuple);
}
