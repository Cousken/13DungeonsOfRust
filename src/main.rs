use std::fs;

mod structs;

fn main() {
    let intro_text: String = fs::read_to_string("assets/intro_file.txt").unwrap();

    println!("{}", intro_text);

    // testing some ownership passing
    let mut counter = structs::Counter::new();

    structs::ThreadRunner::increment_on_thread(1, 10, &mut counter);
}