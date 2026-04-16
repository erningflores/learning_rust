static NUM: i32 = 18;

fn coerce_static<'a>(_: &'a i32) -> &'a i32{
    &NUM
}

fn main() {
    println!("Static lifetime!\n");
    
    {
        let static_string = "I'm in read-only memory";
        println!("static string: {}", static_string);
    }

    {
        let lifetime_num = 9;
        let coerce_static = coerce_static(&lifetime_num);
        println!("coerce_static: {}", coerce_static);
    }

    println!("NUM: {} stays accessible!", NUM);
}
