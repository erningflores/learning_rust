use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber{
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error>{
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

fn main() {
    println!("TryFrom and TryInto!\n");

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    let result1: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result1, Ok(EvenNumber(8)));
    let result2: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result2, Err(()));
}
