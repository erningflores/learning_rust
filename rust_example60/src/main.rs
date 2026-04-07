#[allow(dead_code)]
#[derive(Clone, Copy)]
struct Book{
    author: & 'static str,
    title: & 'static str,
    year: u32,
}

fn borrow_book(book: &Book){
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

fn new_edition(book: &mut Book){
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    println!("Mutability!\n");

    let immutable_book = Book{
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    let mut mutable_book = immutable_book;

    borrow_book(&immutable_book);
    borrow_book(&mutable_book);

    new_edition(&mut mutable_book);
    //new_edition(&mut immutable_book);
}
