use crate::List::*;

enum List{
    Cons(u32, Box<List>),
    Nil, //Nil is a node that signifies the end of linked list. 
}

impl List{
    //creating an empty list
    fn new() -> List{
        Nil
    }

    //adding a new element to list
    fn prepend(self, elem: u32) -> List{
        Cons(elem, Box::new(self))
    }

    //return the length of the list
    fn len(&self) -> u32{
        match *self{
            Cons(_, ref tail) => 1 + tail.len(),
            Nil => 0,
        }
    }
    
    //return a representation of the list as a heap allocated string
    fn stringify(&self) -> String{
        match *self{
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

//two types of constant: const and static
static LANGUAGE: &str = "Rust";
const THRESHOLD: i32 = 10;

fn is_big(n: i32) -> bool{
    n > THRESHOLD
}

fn main() {
    println!("Linked-list via enum!\n");

    let mut list = List::new();

    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());

    //constant
    let n = 16;

    println!("THis is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);
    println!("{} is {}", n, if is_big(n){"big"} else {"small"});

    /*
    //for static
    you can declare it as mut if you want to change it and put it in unsafe block to change it. But making a static mutable is very dangerous, a thread from somewhere could access it. Check the code below on how to implement it.
    static mut THRESHOLD: i32 = 19;
    fn main(){
        println!("{}", THRESHOLD);
        unsafe{
            THRESHOLD = 100;
            println!("{}", THRESHOLD); 
        }
    }
    */
}
