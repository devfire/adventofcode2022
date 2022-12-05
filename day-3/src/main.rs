use std::collections::HashSet;
use std::fs::File;
// use std::hash::Hash;
// use std::hash;
use std::io::{self, prelude::*, BufReader};
use std::vec;
fn main() -> io::Result<()> {

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    // sum of the priorities
    let mut total_value = 0;

    let mut line_count = 0;

    let mut vector_of_sets: Vec<HashSet<char>> = vec![];

    for line in reader.lines().flatten() {
        // assign the line to a hash set
        vector_of_sets.push(add_to_hash_set(line));
        
        line_count += 1;
        
        if line_count > 2 {
            for c in intersection(vector_of_sets) {
                total_value += get_item_value(&c);
            }
            // let c: char = intersection(vector_of_sets).get(value)
            // println!("Intersection of all: {:?}", intersection(vector_of_sets).get(&0));
            vector_of_sets = vec![];
            line_count = 0;
        }
    }

    println!("Total is {}", total_value);

    
    Ok(())
}


fn intersection(mut sets: Vec<HashSet<char>>) -> HashSet<char> {
    if sets.is_empty() {
        return HashSet::new();
    }
    
    if sets.len() == 1 {
        return sets.pop().unwrap();
    }
    
    let mut result = sets.pop().unwrap();
    result.retain(|item| {
        sets.iter().all(|set| set.contains(item))
    });
    result
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