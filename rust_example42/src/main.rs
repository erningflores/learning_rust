//extern crate rary;
mod my;

fn function(){
    println!("called `function()`");
}

fn main() {
    println!("visibility examples in files\n");

    my::function();
    function();
    my::indirect_access();
    my::nested::function();

    //commented since it runs in rustc but not in cargo. maybe misplaced in files
    //rary::public_function();
    //rary::indirect_access();
}
