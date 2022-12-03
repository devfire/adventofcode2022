use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    // File must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let mut running_total = 0;
        let mut previous_total = 0;
        let mut vector: Vec<i32> = Vec::new();


        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(calories) = line {
                // println!("{}", calories);
                match calories.parse::<i32>() {
                    Ok(n) => {
                        running_total = running_total + n;
                        // println!("Running total: {}", running_total);
                    }
                    Err(_) => {
                        // println!("Newline!");
                        if running_total > previous_total {
                            println!("Final running total: {}", running_total);
                            previous_total = running_total;
                            
                        }
                        vector.push(running_total);
                        // reset the count since Err() indicates a new elf
                        running_total = 0;
                    }
                }
            }
        }

        println!("Larget calorie count is {}", previous_total);
        
        // sort by descending order
        vector.sort_by(|a, b| b.cmp(a));

        println!("Top 3 elves: {}", vector[0] + vector[1] + vector[2]);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
