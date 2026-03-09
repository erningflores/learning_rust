/*
use crate::deeply::nested::{
    my_first_function,
    my_second_function,
    AndATraitType,
};
*/
//you can bind it to a different name
use deeply::nested::function as other_function;

fn function(){
    println!("called `function()`");
}

mod deeply{
    pub mod nested{
        pub fn function(){
            println!("called `deeply::nested::function()`");
        }
    }
}


fn main() {
    println!("use\n");

    //my_first_function();
    other_function();
    println!("Entering the block");
    {
        //this use case will shadow the other one
        use crate::deeply::nested::function;
        function();
        println!("Leaving the block");
    }

    function();
}
