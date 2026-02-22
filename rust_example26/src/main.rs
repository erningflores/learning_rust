use std::str::FromStr;

enum Foo{
    Bar,
    Baz,
    Qux(u32),
}

fn get_count_item(s: &str) -> (u64, &str){
    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else{
        panic!("Can't segment count item pair: '{}'", s);
    };
    let Ok(count) = u64::from_str(count_str) else{
        panic!("Can't parse integer: 'count_str'");
    };
    (count, item)
}

fn main() {
    println!("if let, let else, while let\n");

    let number = Some(7);
    let letter: Option<i32> = None;
    let emoticon: Option<i32> = None;

    if let Some(i) = number{
        println!("Matched: {:?}", i);
    }

    if let Some(i) = letter{
        println!("Matched: {:?}", i);
    }else {
        println!("Didn't match a number. Let's go with a letter!");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon{
        println!("Matched: {:?}", i);
    } else if i_like_letters {
        println!("Didn't match a number. Let's go with a letter!");
    } else {
        println!("I don't like letters. Let's go with emoticons :)!");
    }

    //if let used to match any enum value
    let a = Foo::Bar;
    let b = Foo::Baz;
    let c = Foo::Qux(100);

    if let Foo::Bar = a{
        println!("a is foobar");
    }

    if let Foo::Bar = b{
        println!("b is foobar");
    }else {
        println!("b is foobaz");
    }

    if let Foo::Qux(value) = c{
        println!("c is {}", value);
    }

    if let Foo::Qux(_value @ 100) = c{
        println!("c is one hundred");
    }

    assert_eq!(get_count_item("3 chairs"), (3, "chairs"));

    //while let
    let mut optional = Some(0);

    while let Some(i) = optional{
        if i > 9{
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again", i);
            optional = Some(i + 1);
        }
    }
}
