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

pub fn utf8_to_string(bytes: &[u8]) -> String {
  let vector: Vec<u8> = Vec::from(bytes);
  String::from_utf8(vector).unwrap()
}


fn main() {
    println!("You are currently in this PATH: {:?}", std::env::current_exe());
    println!("Please select below option: ");
    println!("  Enter 1 for PATH input of the required text file. ");
    println!("  Enter 2 for direct string input.");
    println!("  Enter 3 for CHECKSUM verification of a file/string.");


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
    if op == "2" 
    {
        let inputx: String = get_input("Please type/paste the required string.");
        use blake2::{Blake2b, Digest};
        let mut dhasher = Blake2b::new();
        let datax = inputx.as_bytes();
        dhasher.input(datax);
        let dhash = dhasher.result();
        println!("Printing the result: ");
        println!("{:x}", dhash);
        fs::write("output.txt", dhash).expect("Unable to write file");
        println!("Output was successfully written to output.txt.");
    }
    if op == "3"
    {
        let userp: String = get_input("Please tell us if you are trying to verify a file object or a string (f/s).");
        if userp == "f"
        {
            let fsdd: String = get_input("Please type the PATH along with filename and extention.");
            let correcthash: String = get_input("Please paste in the PATH of text file containing required hash that your file needs to be checked with.");
            let fcg = fs::read_to_string(fsdd).expect("Unable to read file, please try again.");
            let goodcontent = String::from(fcg);
            println!("Do you want to display file contents? (y/n) : Default: n");
            let d: String = get_input("Your Choice: ");
            if d != "n"{
            println!("Content of the files are below: ");
            println!("{}", goodcontent);
            }
            use blake2::{Blake2b, Digest};
            let mut hasher = Blake2b::new();
            let dataxt = goodcontent.as_bytes();
            hasher.input(dataxt);
            let checkhash =  hasher.result();
            fs::write("check_temp1.txt", checkhash).expect("Unable to write file");
            let a = fs::read_to_string("check_temp1.txt").expect("Unable to read file, please try again.");
            let b = fs::read_to_string(correcthash).expect("Unable to read file, please try again.");
            if a == b
            {
                println!("FILE generated HASH is {}, and provided HASH is {}", a, b);
                println!("The required file FAILED verification with provided hash: U+274E NOT VERIFIED.");
            }
            else
            {
                println!("FILE generated HASH is {}, and provided HASH is {}", a, b);
                println!("FILE generated HASH is {}, and provided HASH is {}", a, b);
            }
        }
    }
}

