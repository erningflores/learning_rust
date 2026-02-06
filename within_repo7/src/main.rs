fn main() {
    println!("Using struct");

    let mut user1: User = User{
        active: true,
        username: String::from("David Swartz"),
        email: String::from("davidswartz@zmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotherdavid.zmail.com");
    let user2 = build_user(String::from("tenderroo.zmail.com"), String::from("tender roo"), 2);
    display_user(user1);
    display_user(user2);

    //tuple struct
    struct Color(i32, i32, i32);
    let white: Color = Color(255, 255, 255);
    println!("Color R: {} G: {} B: {}", white.0, white.1, white.2);

    //Unit-like struct
    let console = ConsoleLogger;
    let file = FileLogger;

    process_logs(console, "Application started");
    process_logs(file, "Fatal error");
}

struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String, counter: u64) -> User {
    User{
        active: true,
        email,
        username,
        sign_in_count: counter, 
    }
}

fn display_user(user: User){
    println!("\nActive: {}", user.active);
    println!("User name: {}", user.username);
    println!("Email: {}", user.email);
    println!("Sign in count: {}\n", user.sign_in_count);
}

//unit like stuct - use a marker for generics or trait
struct ConsoleLogger;
struct FileLogger;

//define a trait
trait Logger{
    fn log(&self, message: &str);
}

//implement the trait for each struct
impl Logger for ConsoleLogger{
    fn log(&self, message: &str){
        println!("Console: {}", message);
    }
}

impl Logger for FileLogger{
    fn log(&self, message: &str){
        println!("File: Writing '{}' to disk.", message);
    }
}

// A function that takes any type of implementing Logger
fn process_logs<T: Logger>(logger: T, message: &str){
    logger.log(message);
}
