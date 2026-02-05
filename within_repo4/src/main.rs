fn main() {
    let s1 = String::from("Rust");
    let l1 = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, l1);

    //there can only one owner
    let s2 = s1;
    //println!("{}", s1); this print will have an error. the ownership was transferred to s2
    println!("{}", s2);

    //references can be both mutable and immutable
    //immutable referencing
    let num1: i32 = 5;
    let num2: &i32 = &num1;
    println!("Value of num1 is {}", num1);
    println!("Value of num2 is {}", num2);

    //mutable referencing
    let mut new_num1: i32 = 100;
    let new_num2: &mut i32 = &mut new_num1;

    *new_num2 += 10;
    *new_num2 -= 25;
    //println!("Value of new_num1 is {}", new_num1);
    //Rust sees you trying to "read" through the owner while a "write" permission is still held by the reference. It blocks this to prevent data races or pointer inconsistency.
    println!("Value of new_num2 is {}", new_num2);

    //solution you let the write finished first before reading the original
    let mut new_try1: i32 = 50;
    {
        let new_try2: &mut i32 = &mut new_try1;

        *new_try2 *= 3;
        println!("Value of new_try2 is {}", new_try2);
    }
    println!("Value of new_try1 is {}", new_try1);

    //===============================================
    //use function to make sure the previous error won't happen since
    // they are in each own scope within each function
    let mut account: BanckAccount = BanckAccount{
        owner: "Alice".to_string(),
        balance: 150.55,
    };

    //immutable borrow to check the balance
    account.check_balance();
    
    //mutable borrow to withdraw money
    account.withdraw(45.5);
    account.check_balance();
}

fn calculate_length(s: &String) -> usize{
    s.len()
}

struct BanckAccount{
    owner: String,
    balance: f64,
}

impl BanckAccount{
    fn withdraw(&mut self, amount: f64){
        println!("Withdrawing {} from account owned by {}", amount, self.owner);
        self.balance -= amount;
    }

    fn check_balance(&self){
        println!("Account owned by {} has a balance of {}", self.owner, self.balance);
    }
}
