use std::fmt;

fn reverse(pair: (i32, bool)) -> (bool, i32){
    let (int_param, bool_param) = pair;
    (bool_param, int_param)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "({}, {})\n({}, {})", self.0, self.1, self.2, self.3)
    }
}

fn transpose(matrix: Matrix) -> Matrix{
    Matrix(matrix.0, matrix.2, matrix.1, matrix.3)
}

fn main() {
    println!("Tuples!\n");

    //more than twelve elements tuple cannot be printed.
    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8, -2i16, -3i32, -4i64, 0.1f32, 0.2f64, 'a', true);
    
    println!("more than twelve elements: {:?}", long_tuple);
    println!("Long_tuple first value: {}", long_tuple.0);
    println!("Long_tuple second value: {}", long_tuple.1);

    //tuple within a tuple
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("The reverse pair is {:?}", reverse(pair));

    //comma is required for one element tuple
    println!("One element tuple: {:?}", (5u32, ));
    //without a comma the parenthesis is like a math grouping
    println!("Just an integer: {:?}", (5u32));

    //tuple can be destructed to create bindings
    let tuple = (1, "hello", 4.5, true);
    let (a, b, c, d) = tuple;
    println!("a={:?}, b={:?}, c={:?}, d={:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
    println!("matrix:\n{}", matrix);

    println!("transpose:\n{}", transpose(matrix));
}
