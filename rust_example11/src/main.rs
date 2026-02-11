enum WebEvent{
    PageLoad,
    PageUnload,
    KeyPress(char),
    Paste(String),
    Click{x: i64, y: i64},
}

fn inspect(event: WebEvent){
    match event{
        WebEvent::PageLoad => println!("Page loaded!"),
        WebEvent::PageUnload => println!("Page unloaded!"),
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        WebEvent::Click{x, y} => println!("clicked at x={}, y={}.", x, y),
    }
}

enum MiniCalculator{
    Add,
    Subtract,
    Multiply,
    Divide,
    Remainder,
}

impl MiniCalculator{
    fn run(&self, x:i32, y: i32) -> i32{
        match self{
            Self::Add => x + y,
            Self::Subtract => x - y,
            Self::Multiply => x * y,
            Self::Divide => x / y,
            Self::Remainder => x % y,
        }
    }
}

fn main() {
    println!("enum!\n");

    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let pressed = WebEvent::KeyPress('x');
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click{x: 20, y: 80};

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);

    //if the enum name is too long, you can use an type alias
    type _Opt = MiniCalculator;
    println!("101 + 6 = {}", _Opt::Add.run(101, 6));
    println!("101 - 6 = {}", _Opt::Subtract.run(101, 6));
    println!("101 * 6 = {}", _Opt::Multiply.run(101, 6));
    println!("101 / 6 = {}", _Opt::Divide.run(101, 6));
    println!("101 % 6 = {}", _Opt::Remainder.run(101, 6));
}
