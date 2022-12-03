use std::fs::File;
use std::io::{self, prelude::*, BufReader};
// use std::thread::current;
// 1 for Rock, 2 for Paper, and 3 for Scissors
// opponent play: A for Rock, B for Paper, and C for Scissors.
// you play:      X for Rock, Y for Paper, and Z for Scissors.
// (0 if you lost, 3 if the round was a draw, and 6 if you won).

// part 2:
// X means to lose, Y means to draw, and Z means to win.

fn main() -> io::Result<()> {
    const LOSS: i32 = 0;
    const DRAW: i32 = 3;
    const WIN: i32 = 6;

    const ROCK: i32 = 1;
    const PAPER: i32 = 2;
    const SCISSORS: i32 = 3;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut total_score = 0;

    for line in reader.lines() {
        
        if let Ok(mut current_line) = line {
            current_line.retain(|c| !c.is_whitespace());
            // println!("{}", current_line);
            match current_line.as_str() {
                // part 2
                "AX" => total_score += SCISSORS + LOSS, // x is to lose
                "BY" => total_score += PAPER + DRAW, // y is to draw
                "CZ" => total_score += ROCK + WIN, // z is to win
                // 
                "AY" => total_score += ROCK + DRAW, // y is to draw
                "BZ" => total_score += SCISSORS + WIN,
                "CX" => total_score += PAPER + LOSS,
                // 
                "AZ" => total_score += PAPER + WIN,
                "BX" => total_score += ROCK + LOSS,
                "CY" => total_score += SCISSORS + DRAW,
                _ => println!("Unexpected combo"),
            }
        }        
    }

    println!("Total score: {}", total_score);

    Ok(())
}