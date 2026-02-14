use std::convert::From;
//se std::convert::Into;

#[derive(Debug)]
struct Number{
    value: i32,
}

//defining a conversion for our own type
impl From<i32> for Number{
    fn from(item: i32) -> Self{
        Number{
            value: item
        }
    }
}

//into is the reciprocal of from but do not implement this. it is already given by rust
/*
impl Into<Number> for i32{
    fn into(self) -> Number{
        Number {value : self}
    }
}
*/
fn main() {
    println!("Conversion");
    /*
        1. primitive types through casting
        2. custom types like struct or enum by the use of traits
        3. generic conversion will use the From and Into traits
    */

    //'From' - allows you to define from another type
    let my_str = "hello";   //this is a string slice
    let my_string = String::from(my_str);
    println!("{}, i was converted from string slice to string", my_string);

    let num1 = Number::from(30);
    println!("My number1 is: {:?}", num1);

    let my_num = 10;
    let num2: Number = my_num.into();
    println!("My number2 is: {:?}", num2);
}
