fn function(){
    println!("called `function()`");
}

mod cool{
    pub fn function(){
        println!("called `cool::function()`");
    }
}

mod my{
    fn function(){
        println!("called `my::function()`");
    }

    mod cool{
        pub fn function(){
            println!("called `my::cool::function()`");
        }
    }

    pub fn indirect_call(){
        print!("called `my::indirect_call()`, that\n");

        //same result since self refers to the current module scope
        self::function();
        function();

        self::cool::function();

        //super refers to the parent scope outside the `my` module
        super::function();

        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}

fn main() {
    println!("super and self\n");

    my::indirect_call();
}
