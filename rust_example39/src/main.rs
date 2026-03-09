mod my{
    pub struct OpenBox<T>{
        pub contents: T,
    }

    pub struct ClosedBox<T>{
        contents: T,
    }

    impl<T> ClosedBox<T>{
        pub fn new(contents: T) -> ClosedBox<T>{
            ClosedBox{
                contents: contents,
            }
        }
    }
}

fn main() {
    println!("struct visibility!\n");

    //pub struct with public fields can be constructed as this
    let open_box = my::OpenBox{ contents: "public information" };
    println!("The open box contains: {}", open_box.contents);

    //pub struct with private fields can be constructed below
    let _closed_box = my::ClosedBox::new("Classified information");
    // the private field of that public struct cannot be accessed.
    //println!("The closed box contains: {}", _closed_box.contents);
}
