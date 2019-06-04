use std::fs;
use simple_user_input::get_input;


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
        let content = String::from(data);
        println!("Do you want to display file contents? (y/n) : Default: n");
        let d: String = get_input("Your Choice: ");
        if d != "n"{
        println!("Content of the files are below: ");
        println!("{}", content);
        }
        use blake2::{Blake2b, Digest};
        let mut hasher = Blake2b::new();
        let datax = content.as_bytes();
        hasher.input(datax);
// `input` can be called repeatedly
        //hasher.input("String data".as_bytes());
// Note that calling `result()` consumes hasher
        let hash = hasher.result();
        println!("Printing the result: ");
        println!("{:x}", hash);
        fs::write("output.txt", hash).expect("Unable to write file");
        println!("Output was successfully written to output.txt.");
    }
}

