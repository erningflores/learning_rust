use std::fmt::Debug;

trait PrintInOption{
    fn print_in_option(self);
}

impl <T> PrintInOption for T where Option<T>: Debug{
    fn print_in_option(self){
        println!("{:?}", Some(self));
    }
}

//newtype idiom - guarantees that the right type of value is supplied
struct Years(i64);
struct Days(i64);

impl Years{
    pub fn to_days(&self) -> Days{
        Days(self.0 * 365)
    }
}

impl Days{
    pub fn to_years(&self) -> Years{
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool{
    age.0 >= 18
}

fn main() {
    println!("where clause!\n");

    let vec = vec![1, 2, 3];
    vec.print_in_option();

    let age = Years(25);
    let age_days = age.to_days();

    println!("Is an adult: {}", is_adult(&age));
    println!("Is an adult: {}", is_adult(&age_days.to_years()));
    
    //to obtain the newtype's value, you use tuple or destructuring syntax
    let years = Years(42);
    let years_as_primitive: i64 = years.0; //tuple
    let Years(years_as_primitive2) = years; //destructuring
    println!("years as primitve: {}", years_as_primitive);
    println!("years as primitive2: {}", years_as_primitive2);
}
