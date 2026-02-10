use std::mem;

fn analyze_slice(slice: &[i32]){
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements.", slice.len());
    //println!("The slice elements are: {:?}", slice);
}

fn add(a: i32, b: i32) -> i32{
    a + b
}

fn main() {
    println!("Arrays and Slice!\n");

    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("First element of xs is {}", xs[0]);
    println!("Second element of xs is {}", xs[1]);
    println!("The number of elements in xs is {}", xs.len());

    //arrays are stack allocated
    println!("Arrays xs occupies {} bytes", mem::size_of_val(&xs));
    //Arrays can be automatically borrowed as slices
    println!("Borrow the whole array as a slice:");
    analyze_slice(&xs);

    //slices can point to a section of an  array
    println!("Borrow a section of an array as a slice:");
    analyze_slice(&ys[1 .. 4]); 

    //empty slice
    let empty_array: [u32; 0] = [];
    let _assert_1 = assert_eq!(&empty_array, &[]);
    println!("assert1: {:?}", _assert_1);
   //assert return nothing just a unit type
   //it will return only if it fails 

    let result = add(2, 3);
    let _assert_2 = assert_eq!(result, 5);
    println!("assert2: {:?}", _assert_2);

    //out of bounds indexing of an array causes compile time error
    //if  you remove the + 1 the None will not trigger.
    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{} : {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }
}
