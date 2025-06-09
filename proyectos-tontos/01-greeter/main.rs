use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let your_name = &args[1];
        println!("Hello, {}!", your_name);
        println!("Let's build something cool with Rust!");
    } else {
        println!("Please provide exactly one string as a command line argument.");
    }
}
