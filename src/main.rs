//use std::env;
use std::fs;
use simple_user_input::get_input;
//use std::path::PathBuf;

mod simple_user_input {
    use std::io;
    pub fn get_input(prompt: &str) -> String{
        println!("{}",prompt);
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_goes_into_input_above) => {},
            Err(_no_updates_is_fine) => {},
        }
    input.trim().to_string()
    }
}

fn main() {
    println!("You are currently in this PATH: {:?}", std::env::current_exe());
    println!("Please select below option: ");
    println!("  Enter 1 for PATH input of the required text file. ");
    println!("  Enter 2 for direct string input.");

    let op: String = get_input("Your choice: ");
    println!("Your choice was {}", op);

    if op == "1"
    {
        let input: String = get_input("Please type the PATH along with filename and extention.");
        let data = fs::read_to_string(input).expect("Unable to read file, please try again.");
        println!("{}"); 

    }

    
  
}

