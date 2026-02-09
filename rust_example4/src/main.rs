use std::fmt;

struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        let vec = &self.0;
        let last_index = vec.len() - 1;
        //println!("last index: {}", last_index);
        write!(f, "["  )?;
        /*
        for (index, v) in vec.iter().enumerate(){
            if index !=0 {
                write!(f, ", ")?;
            }
            write!(f, "{}", v)?;
        }
        */
        //answer to the activity is below
        for (index, v) in vec.iter().enumerate(){
            if index == last_index{
                write!(f, "{}: {}", index, v)?;
            }else {
                write!(f, "{}: {}, ", index, v)?;
            }
        }
        write!(f, "]")
    }
}

//activity
//the output should be like this --> [0: 1, 1: 2, 2: 3]
//current output --> [1, 2, 3]

fn main() {
    println!("? operator to handle fmt::Display elements sequentially!");

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

}
