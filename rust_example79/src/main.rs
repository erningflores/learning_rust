#[derive(Debug, Clone, Copy)]
struct Unit;

//Box<T> cannot be copy since it is not a copy
#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    println!("Clone and Copy!\n");

    let unit = Unit;
    //this is an implicit copy
    let copied_unit = unit;

    println!("original: {:?}", unit);
    println!("copy: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("original: {:?}", pair);
    //pair does not implement a copy so, this is a moved.
    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    let cloned_pair = moved_pair.clone();
    drop(moved_pair);
    println!("cloned: {:?}", cloned_pair);
}
