use std::fs;
use std::collections::HashSet;

fn main() {
    let distinct_char = 14;
    let mut buffer: Vec<char> = Vec::new();

    let contents = fs::read_to_string("signal").expect("Failed to read file");

    let mut index = 1;
    for c in contents.chars() {
        buffer.push(c);

        if buffer.len() >= distinct_char {
            let mut i = 0;
            let mut different = true;
            let mut set: HashSet<char> = HashSet::new();

            while i < distinct_char {
                println!("{} {} {}", index, buffer[buffer.len() - i - 1], i);
                if set.contains(&buffer[buffer.len() - i - 1]) {
                    println!("FOUND");
                    different = false;
                    break;
                }
                else {
                    set.insert(buffer[buffer.len() - i - 1]);
                }

                i += 1;
            }

            if different {
                break;
            }
        }

        index += 1;
    }

    println!("index: {}", index);
}
