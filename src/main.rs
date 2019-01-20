use std::fs;

fn main() {
    let intro_text: String = fs::read_to_string("assets/intro_file.txt").unwrap();

    println!("{}", intro_text);
}
