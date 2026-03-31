//Resource Acquisition Is Initialization
fn create_box(){
    //allocate an integer on the heap
    let _box1 = Box::new(3i32);

    //_box1 is destroyed here and memory gets freed
}

struct ToDrop;

impl Drop for ToDrop{
    fn drop(&mut self){
        println!("ToDrop is being dropped");
    }
}


fn main() {
    println!("Hello, world!");

    //allocated an integer on the heap
    let _box2 = Box::new(5i32);

    {
        //allocate an integer on the heap
        let _box3 = Box::new(4i32);

        //_box3 is destroyed here and memory gets freed
    }

    for _ in 0u32..1_000{
        create_box();
    }

    let _x = ToDrop;
    println!("Made a ToDrop!");

    //_box2 is destroyed here and memory gets freed
}
