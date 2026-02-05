fn main() {
    hello_world();
    tell_height(182);
    human_id("David Swartz", 60, 183.5);

    //========================
    //expression -- it returns something
    let answer_expr: i32 = {
        let price: i32 = 5;
        let qty: i32 = 10;
        //==adding a semicolon below would become a statement, a unit ().
        price * qty 
    };
    println!("The result is {}", answer_expr);

    let z = add_numbers(75, 40);
    println!("z: {}", z);

    println!("Adding 33 and 53 is {}", add_numbers(33, 53));

    let weight: f64 = 80.5;
    let height: f64 = 1.82;
    let bmi: f64 = calculate_bmi(weight, height);
    //setting the format to two decimal place
    println!("Your bmi is {:.2}", bmi);
}

fn hello_world(){
    println!("Hello, world!");
}

fn tell_height(height: u32){
    println!("My height is {}", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {}. And i am {} years old. My height is {} cm.", name, age, height);
}

fn add_numbers(a: i32, b: i32) -> i32{
    a + b
}

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}