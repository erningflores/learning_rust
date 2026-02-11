#![allow(dead_code)]

enum Stage{
    Beginner,
    Advance,
}

enum Role{
    Student,
    Teacher,
}

//C like enums
enum Number{
    Zero,
    One,
    Two,
}

enum Color{
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    println!("use declaration!\n");

    //manual scoping
    use Stage::{ Beginner, Advance };
    //automatic scoping
    use Role::*;

    //Equivalent to Stage::Beginner
    let stage = Beginner;
    //Equivalent to Role::Student
    let role = Student;

    match stage{
        Beginner => println!("Beginners are starting their learning journey!"),
        Advance => println!("Advance learners are mastering their studies..."),
    }

    match role{
        Student => println!("Students are acquiring knowledge!\n"),
        Teacher => println!("Teachers are spreading knowledge!\n"),
    }

    println!("Zero is {}", Number::Zero as i32);
    println!("One is {}\n", Number::One as i32);

    println!("Roses are #{:06x}", Color::Red as u32);
    println!("violets are #{:06x}", Color::Blue as u32);

}
