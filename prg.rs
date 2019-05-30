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
    //println!("{:?}", std::env::current_exe());
   // let input: String = get_input("Please type path...");
   // let data = fs::read_to_string(input).expect("Unable to read file");
    //println!("{}", data);
    let cwd = env::current_dir().unwrap();
    let my_str = cwd.into_os_string().into_string().unwrap();
    println!("{}", mystr);
}

