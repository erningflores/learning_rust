pub trait Iterator{
    type Item;

    fn any<F>(&mut self, f: F) -> bool where
        F: FnMut(Self::Item) -> bool;
}

fn main() {
    println!("iterator");

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    //iter() just borrowed the vec1 so it can be use again
    println!("vec1 len: {}", vec1.len());
    println!("First element of vec1: {}", vec1[0]);

    //into_iter() does move the vec2 so it cannot be used again
    //println!("vec2 len: {}", vec2.len());
    //println!("First element of vec2: {}", vec2[0]);

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("2 in array1: {}", array1.iter().any(|&x| x == 2));
    println!("2 in array2: {}", IntoIterator::into_iter(array2).any(|x| x == 2));
}
