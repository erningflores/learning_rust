#[derive(Debug)]
struct A;

#[derive(Debug)]
struct S(A);

#[derive(Debug)]
struct SGen<T>(T);

//not generic function
fn reg_fn(_s: S){
    println!("called reg_fn: {:?}", _s);
}

//not generic funciton
fn gen_spec_t(_s: SGen<A>){
    println!("called gen_spec_t: {:?}", _s);
}

//not generic function
fn gen_spec_i32(_s: SGen<i32>){
    println!("called gen_spec_i32: {:?}", _s);
}

fn generic<T: std::fmt::Debug>(_s: SGen<T>){
    println!("called generic: {:?}", _s);
}


fn main() {
    println!("Generic function!\n");

    reg_fn(S(A));
    gen_spec_t(SGen(A));
    gen_spec_i32(SGen(6));

    //explicitly specify
    generic::<char>(SGen('a'));
    //implicitly specify 
    generic(SGen('c'));
}
