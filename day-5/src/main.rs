use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    // let mut matrix: Vec<Vec<char>>;

    // Open the file in read-only mode
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // println!("Line {}: {}", line_counter, line);

        for (char_counter, c) in line.chars().enumerate() {
            if is_capital(c) {
                print!("Pos:{}\tLetter:{}\t", char_counter, c);
            }
        }
        println!();
        if has_number(line.as_str()) {
            break;
        }
    }

    Ok(())
}

fn has_number(s: &str) -> bool {
    s.chars().any(|c| c.is_numeric())
}

fn is_capital(c: char) -> bool {
    c.is_uppercase()
}
