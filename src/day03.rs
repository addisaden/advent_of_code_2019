use std::env;
use std::fs;

fn data() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        fs::read_to_string(String::from(args[2].to_string())).expect("")
    } else {
        println!("USAGE: {} {} <filename-of-input>", args[0], args[1]);
        String::from("")
    }
}

pub fn print_result() {
    println!("{}", data());
    println!("Not solved yet.");
}
