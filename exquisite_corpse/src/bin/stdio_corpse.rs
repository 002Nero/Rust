
use std::io::{self, Write};
use exquisite_corpse::add_line_and_make_response;

fn main() {
    let mut text = String::new();
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).expect("Failed to read line");
        let line = line.trim();
        if line.is_empty() {
            break;
        }
        text = add_line_and_make_response(line, &mut text);
        println!("{}", text);
    }
}