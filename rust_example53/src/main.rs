use std::marker::PhantomData;

#[derive(PartialEq)]    //allow equality test for this type
struct PhantomTuple<A, B>(A, PhantomData<B>);   //generic A with hidden B

#[derive(PartialEq)]
struct PhantomStruct<A, B>{
    first: A,
    phantom: PhantomData<B>,
}

//note: storage is allocated for generic A but not for phantom B

fn main() {
    println!("Phantom type parameter!\n");

    let _tuple1: PhantomTuple<char, f32> = PhantomTuple('Q', PhantomData);
    let _tuple2: PhantomTuple<char, f64> = PhantomTuple('Q', PhantomData);

    //mismatch because of phantom data
    //println!("_tuple1 ==  _tuple2 yields: {}", _tuple1 == _tuple2);

    let _struct1: PhantomStruct<char, f32> = PhantomStruct{
        first: 'Q',
        phantom: PhantomData,
    };

    let _struct2: PhantomStruct<char, f64> = PhantomStruct{
        first: 'Q',
        phantom: PhantomData,
    };

    //mistmatch becuase of mistamatch phantom data
    //println!("_struct1 == struct2 yields: {}", _struct1 == _struct2);
}
