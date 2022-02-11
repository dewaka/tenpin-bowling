use crate::bowling::{Bowling, BowlingError, TenPinBowling};
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
            Err(err) => {
                println!("{}", err);
                break;
            }
        }

        println!("Score: {}", bowling.score());
    }
}

fn do_roll(bowling: &mut TenPinBowling) -> Result<bool, BowlingError> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin
        .read_line(&mut line)
        .map_err(|_| BowlingError::IOError)?;
    let line = line.trim();
    if line.is_empty() {
        Ok(false)
    } else {
        let pins = line.parse::<i32>().map_err(|_| {
            BowlingError::InvalidRoll(format!("Error parsing pins. Should be a number"))
        })?;
        bowling.roll(pins)?;
        Ok(true)
    }
}
