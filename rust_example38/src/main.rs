mod my_mod{
    //by default, all items inside the module are private.
    fn private_function(){
        println!("called `my_mod::private_function()`");
    }

    //use pub modifier to override private visibility
    pub fn function(){
        println!("called `my_mod::function()`");
    }

    //items can access other items in the same module
    pub fn indirect_access(){
        print!("called `my_mod::indirect_access()`, that \n>");
        private_function();
    }

    //modules can be nested
    pub mod nested{
        pub fn function(){
            println!("called `my_mod::nested::function()`");
        }

        #[allow(dead_code)]
        fn private_function(){
            println!("called `my_mod::nested::private_function()`");
        }

        //function declared using pub(in path) are only visible within the given path
        pub(in crate::my_mod) fn public_function_in_my_mod(){
            print!("called `my_mod::nested::public_function_in_my_mod()`, that \n>");
            public_function_in_nested();
        }

        //function declared pub(self) are only visible within the module
        pub(self) fn public_function_in_nested(){
            println!("called `my_mod::nested::public_function_in_nested()`");
        }

        //function declared pub(super) are only visible within the parent module
        pub(super) fn public_function_in_super_mod(){
            println!("called `my_mod::nested::public_function_in_super_mod()`");
        }
    }

    pub fn call_public_function_in_my_mod(){
        print!("called `my_mod::call_public_function_in_my_mod()`, that \n>");
        nested::public_function_in_my_mod();
        print!("> ");
        nested::public_function_in_super_mod();
    }

    //function declared pub(crate) makes only visible within the crate
    pub(crate) fn public_function_in_crate(){
        println!("called `my_mod::public_function_in_crate()`");
    }

    //nested modules follow the same rules for visibilty
    mod private_nested{
        #[allow(dead_code)]
        pub fn function(){
            println!("called my_mod::private_nested::function()");
        }

        //private parents will still restrict a child 
        // declared as visible within a bigger scope
        #[allow(dead_code)]
        pub(crate) fn restricted_function(){
            println!("called `my_mod::private_nested::restricted_function()`");
        }
    }
}

fn function(){
    println!("called `function()`");
}

fn main() {
    println!("visibility modifiers!\n");

    function();
    my_mod::function();

    //public items including those inside the nested modules
    //can be accessed from outside by parent module
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    //pub(crate) can be called anywhere within the crate
    my_mod::public_function_in_crate();

    /*
    //below are not allowed to be called since it is private
    //my_mod::nested::public_function_in_my_mod();
    //my_mod::private_function();
    //my_mod::nested::private_function();
    //my_mod::private_nested::private_function();
    //my_mod::private_nested::restricted_function();
    */
}
