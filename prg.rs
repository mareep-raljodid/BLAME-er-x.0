use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn read_a_file(path: String){
let data = fs::read_to_string(path).expect("Unable to read file");
    println!("{}", data);
}

fn main() {
    let path: String = get_input("Please type something...");
    read_a_file(path);
}