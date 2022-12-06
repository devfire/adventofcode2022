// use std::collections::btree_map::Range;

use std::fs::File;
use std::io::{BufRead, BufReader};
// use std::vec;

fn main() {
    let mut count = 0;

    // Open the file in read-only mode
    let file = File::open("input.txt").expect("Failed to open file");
    let reader = BufReader::new(file);

    // Read the file line by line
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // println!("{}", line);

        let vector_pair: Vec<&str> = line.split(',').collect();
        let first_pair = vector_pair[0];
        let second_pair = vector_pair[1];

        // println!("First: {} second: {}", first_pair, second_pair);

        if is_range_subrange(create_range(first_pair), create_range(second_pair)) {
            println!("overlap! {} {}", first_pair, second_pair);

            count += 1;
        }
    }
    println!("Total count is {}", count);
}

fn create_range(string: &str) -> std::ops::Range<i32> {
    // Split the string into individual words
    let words: Vec<&str> = string.split('-').collect();

    // Parse each word as an integer
    let mut numbers = Vec::new();
    for word in words {
        let number = word.parse::<i32>().unwrap();
        numbers.push(number);
    }

    // Find the minimum and maximum values in the vec
    let min = numbers
        .iter()
        .min()
        .expect("Didn't get a digit as expected");
    let max = numbers
        .iter()
        .max()
        .expect("Didn't get a digit as expected");

    // Create the range using the min and max values
    *min..*max
}

fn is_range_subrange(range1: std::ops::Range<i32>, range2: std::ops::Range<i32>) -> bool {
    // Check if range1 is completely within range2
    (range1.start >= range2.start && range1.end <= range2.end)
        || (range2.start >= range1.start && range2.end <= range1.end)
        // Check if either range starts within the other range
        || (range1.start >= range2.start && range1.start <= range2.end) || (range2.start >= range1.start && range2.start <= range1.end)
        // Check if either range ends within the other range
        || (range1.end >= range2.start && range1.end <= range2.end) || (range2.end >= range1.start && range2.end <= range1.end)
}
