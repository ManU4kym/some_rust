use std::{env::args, process::exit};

fn main() {
    let args: Vec<String> = args().collect();
    let name = match args.get(1) {
        Some(name) => name,
        None => {
            println!("Please provide your name");
            exit(1);
        }
    };
    let profession = match args.get(2) {
        Some(profession) => profession,
        None => {
            println!("Please provide your profession");
            exit(1);
        }
    };
    println!(
        "My name is {}, and the profession that i'm in is{}",
        name, profession
    );
}
