/*
pub trait Iterator{
    type Item;
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        P:: FnMut(&Self::Item) -> bool;
}
*/
fn main() {
    println!("Iterator find!\n");

    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let array1 = [1, 2, 3];
    let array2 = [4, 5, 6];

    println!("Find 2 in array1: {:?}", array1.iter().find(|&&x| x == 2));
    println!("Find 2 in array2: {:?}", IntoIterator::into_iter(array2).find(|&x| x == 2));

    //find gives you a reference to the item
    //position gives you the index of the item
    let vec3 = vec![1, 9, 3, 3, 13, 2];
    
    let index_even = vec3.iter().position(|&x| x % 2 == 0);
    assert_eq!(index_even, Some(5));

    let index_negative = vec3.into_iter().position(|x| x < 0);
    assert_eq!(index_negative, None);
}
