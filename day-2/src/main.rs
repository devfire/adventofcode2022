use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// use std::thread::current;
// 1 for Rock, 2 for Paper, and 3 for Scissors
// opponent play: A for Rock, B for Paper, and C for Scissors.
// you play:      X for Rock, Y for Paper, and Z for Scissors.
// (0 if you lost, 3 if the round was a draw, and 6 if you won).


fn main() -> io::Result<()> {
    const LOSS: i32 = 0;
    const DRAW: i32 = 3;
    const WIN: i32 = 6;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {
        
        if let Ok(mut current_line) = line {
            current_line.retain(|c| !c.is_whitespace());
            // println!("{}", current_line);
            match current_line.as_str() {
                // draw
                "AX" => total_score += 1 + DRAW,
                "BY" => total_score += 2 + DRAW,
                "CZ" => total_score += 3 + DRAW,
                // win
                "AY" => total_score += 2 + WIN,
                "BZ" => total_score += 3 + WIN,
                "CX" => total_score += 1 + WIN,
                // loss
                "AZ" => total_score += 3 + LOSS,
                "BX" => total_score += 1 + LOSS,
                "CY" => total_score += 2 + LOSS,
                _ => println!("Unexpected combo"),
            }
        }        
    }

    println!("Total score: {}", total_score);

    Ok(())
}