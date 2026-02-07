fn main() {
    println!("Using Vectors - a growable array");

    let mut _v: Vec<i32> = Vec::new();
    //another option of declaring vector
    //let _the_vec: Vec<i32> = vec![1, 2, 3];

    _v.push(5);
    _v.push(6);
    _v.push(7);
    _v.push(8);
    _v.push(9);

    println!("{:?}", _v);

    //direct indexing
    let third: &i32 = &_v[2];
    println!("The third element: {third}");

    //using get but the result is Option<T>. Take note of the &i32. just referencing
    let fourth: Option<&i32> = _v.get(3);
    match fourth {
        Some(fourth) => println!("The fourth element from the get method: {}", fourth),
        None => println!("There is no fourth element"),
    }
    
}
