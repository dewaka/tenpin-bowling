use crate::bowling::{Bowling, TenPinBowling};
use std::io;

mod bowling;

fn main() {
    let mut bowling = TenPinBowling::new();
    loop {
        match do_roll(&mut bowling) {
            Ok(cont) => {
                if !cont {
                    break;
                }
            }
            Err(msg) => {
                println!("{}", msg);
                break;
            }
        }

        println!("Score: {}", bowling.score());
    }
}

fn do_roll(bowling: &mut TenPinBowling) -> Result<bool, String> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .read_line(&mut line)
        .map_err(|_| format!("Error reading input line"))?;
    let line = line.trim();
    if line.is_empty() {
        Ok(false)
    } else {
        let pins = line
            .parse::<i32>()
            .map_err(|_| format!("Error parsing score. Should be a number"))?;
        bowling.roll(pins)?;
        Ok(true)
    }
}
