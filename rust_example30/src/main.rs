fn main() {
    println!("Closures capturing variables!\n");

    use std::mem;

    let color = String::from("green");
    /*
    println! requires only immutable reference so it doesn't impose anything
    restrictive
    */
    let print = || println!("`color`: {}", color);
    //calling the closure using the borrow
    print();

    //color can be borrowed again since the closure only holds an immutable reference
    let _reborrow = &color;
    print();

    //a move or reborrow is allowed until the final use of print
    let _color_moved = color;

    let mut count = 0;
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    //calling the closure
    inc();
    /*
    &count is not allowed
    */
    //let _reborrow = &count;
    //inc();

    //this was allowed but you cannot use closure after it which is inc()
    let _count_reborrowed = &mut count;
    
    //a non copy type
    let movable = Box::new(3);
    //a copy type would copy into the closure
    //a non copy must move so movable immediately moves into the closure.
    let consume = || {
        println!("`movalble`: {}", movable);
        mem::drop(movable);
    };
    //using this closure, drops the variable, so it can only be used once.
    consume();

    let haystack = vec![1, 2, 3];
    let contains = move | needle | haystack.contains(needle);

    println!("{}", contains(&1));
    println!("{}", contains(&4));

    //error since borrow checker does not allow the reuse of variable after it has moved.
    //println!("There're {} elements in vec", haystack.len());
}
