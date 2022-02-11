use crate::bowling::{Bowling, TenPinBowling};
use std::io;

mod bowling;

fn main() {
    let mut bowling = TenPinBowling::new();
    loop {
        let ok = do_roll(&mut bowling);
        if ok.is_err() {
            break;
        }
        println!("Score: {}", bowling.score());
    }
}

fn do_roll(bowling: &mut TenPinBowling) -> Result<(), ()> {
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.read_line(&mut line).map_err(|_| ())?;
    if line.is_empty() {
        Err(())
    } else {
        let pins = line.trim().parse::<i32>().map_err(|_| ())?;
        Ok(bowling.roll(pins))
    }
}
