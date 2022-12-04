use std::collections::HashSet;
use std::fs::File;
// use std::hash::Hash;
// use std::hash;
use std::io::{self, prelude::*, BufReader};
fn main() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // sum of the priorities
    let mut total_value = 0;

    for line in reader.lines().flatten() {
        // calculate the middle of the string
        let middle = line.len() / 2;

        // split the string down the middle
        let (first, second) = line.split_at(middle);
        
        // assign the split strings to two hash sets
        let hash_set_first = add_to_hash_set(first.to_string());
        let hash_set_second = add_to_hash_set(second.to_string());

        for item in hash_set_first.intersection(&hash_set_second) {
            total_value += get_item_value(item);
            println!("Common item: {} value: {}", item, total_value);
        }
    }

    
    Ok(())
}

fn add_to_hash_set (line: String) -> HashSet<char> {
    let mut hash_set: HashSet<char> = HashSet::new();
    for c in line.chars() {
        hash_set.insert(c);
    }
    hash_set
}

fn get_item_value (c: &char) -> u32 {
    if c.is_ascii_lowercase() {
        (*c as u32) - 96
    } else {
        (*c as u32) - 38
    }
}