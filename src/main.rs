use std::{env, io::BufRead};
use ctrlc;

fn rot13(stringtorot: &String) -> String {
    let mut result = String::with_capacity(stringtorot.capacity());
    for character in stringtorot.bytes() {
        // The problem with this program is that b'M' is 77 so the character is taking the
        // -13 path everytime because of that.
        if character >= b'a' && character <= b'z' {
            if character > b'm' {
                result.push((character-13u8) as char);
            }
            else {
                result.push( (character+13u8) as char);
            }
        } else if character >= b'A' && character <= b'Z' {
            if character > b'M' {
                result.push((character-13u8) as char);
            }
            else {
                result.push( (character+13u8) as char);
            }
        } else {
            result.push(character as char);
        }
    }
    result
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.contains(&String::from("-h")) || args.contains(&String::from("--help")) {
        print!("This program performs the ROT13 cipher (rotate by 13) on stdin and outputs to stdout.");
        std::process::exit(0);
    }
    ctrlc::set_handler(move || {
        std::process::exit(0);
    })
    .expect("Error setting Ctrl-C handler.");
    println!("This program performs rot13 cipher. Press Ctrl-C to exit");
    let mut input = String::new();
    let mut stdin_handle = std::io::stdin().lock();
    loop {
        input.clear();
        match stdin_handle.read_line(&mut input) {
            Ok(_) => (),
            Err(_) => panic!("Failed to read input."),
        }
        
        println!("{}", rot13(&input));
    }
}
