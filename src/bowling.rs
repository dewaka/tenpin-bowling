use thiserror::Error;

#[derive(Error, Debug, Eq, PartialEq)]
pub enum BowlingError {
    #[error("invalid roll")]
    InvalidRoll(String),
    #[error("invalid frame")]
    InvalidFrame(String),
    #[error("game already finished")]
    GameFinished(String),
    #[error("invalid pins error")]
    InvalidPins,
    #[error("bonus error")]
    BonusError,
    #[error("io error")]
    IOError,
}

pub trait Bowling {
    fn roll(&mut self, pins: i32) -> Result<(), BowlingError>;
    fn score(&self) -> i32;
    fn finished(&self) -> bool;
}

static STRIKE: i32 = 10;
static MAX_FRAMES: usize = 10;

#[derive(Debug)]
struct Frame {
    rolls: Vec<i32>,
    bonus: Vec<i32>,
    score: i32,
}

impl Frame {
    fn with_roll(pins: i32) -> Result<Self, BowlingError> {
        if pins > STRIKE {
            Err(BowlingError::InvalidRoll(format!(
                "Invalid pins for frame: {}",
                pins
            )))
        } else {
            Ok(Self {
                rolls: vec![pins],
                bonus: vec![],
                score: 0,
            })
        }
    }

    fn roll(&mut self, pins: i32, last: bool) -> Result<(), BowlingError> {
        let sum = self.roll_sum() + pins;

        if last {
            if sum > STRIKE * 3 {
                return Err(BowlingError::InvalidFrame(format!(
                    "Last frame cannot be more than a {}",
                    3 * STRIKE
                )));
            }
        } else if sum > STRIKE {
            return Err(BowlingError::InvalidFrame(format!(
                "Current frame cannot be more than a {}",
                STRIKE
            )));
        }

        self.rolls.push(pins);
        if self.can_score() {
            self.score = self.roll_sum();
        }
        Ok(())
    }

    fn bonus(&mut self, pins: i32) -> Result<(), BowlingError> {
        if pins > STRIKE {
            return Err(BowlingError::InvalidPins);
        }

        if self.bonus_complete() {
            Err(BowlingError::BonusError)
        } else {
            Ok(self.bonus.push(pins))
        }
    }

    fn bonus_complete(&self) -> bool {
        if self.strike() {
            self.bonus.len() == 2
        } else if self.spare() {
            self.bonus.len() == 1
        } else {
            true
        }
    }

    fn complete(&self, last: bool) -> bool {
        if last {
            if self.spare() || self.strike() {
                self.rolls.len() == 3
            } else {
                self.rolls.len() == 2
            }
        } else {
            self.strike() || self.rolls.len() == 2
        }
    }

    fn strike(&self) -> bool {
        match self.rolls.last() {
            None => false,
            Some(&r) => r == STRIKE,
        }
    }

    fn spare(&self) -> bool {
        self.rolls.len() == 2 && self.roll_sum() == STRIKE
    }

    fn total(&self) -> i32 {
        self.roll_sum() + self.bonus_sum()
    }

    fn roll_sum(&self) -> i32 {
        self.rolls.iter().sum()
    }

    fn bonus_sum(&self) -> i32 {
        self.bonus.iter().sum()
    }

    fn can_score(&self) -> bool {
        !self.spare() && !self.strike()
    }
}

#[derive(Debug)]
pub struct TenPinBowling {
    frames: Vec<Frame>,
    current_score: i32,
}

impl TenPinBowling {
    fn update_frames(&mut self, pins: i32) -> Result<(), BowlingError> {
        let last_frame = self.last_frame();

        match self.current_frame_mut() {
            None => self.add_new_frame(pins),
            Some(frame) => {
                if frame.complete(last_frame) {
                    self.add_new_frame(pins)?;
                } else {
                    frame.roll(pins, last_frame)?;
                }
                Ok(self.update_bonus(pins))
            }
        }
    }

    fn last_frame(&self) -> bool {
        self.frames.len() == MAX_FRAMES
    }

    fn update_bonus(&mut self, pins: i32) {
        if self.frames.len() < 2 {
            return;
        }

        let to = self.frames.len() - 1;
        self.frames.iter_mut().take(to).for_each(|f| {
            if !f.bonus_complete() {
                f.bonus(pins).unwrap();
            }
        });
    }

    fn add_new_frame(&mut self, pins: i32) -> Result<(), BowlingError> {
        let frame = Frame::with_roll(pins)?;
        Ok(self.frames.push(frame))
    }

    fn update_score(&mut self) {
        self.current_score = self.frames.iter().map(|f| f.total()).sum();
    }

    fn current_frame_mut(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut()
    }

    pub fn new() -> Self {
        Self {
            frames: vec![],
            current_score: 0,
        }
    }
}

impl Bowling for TenPinBowling {
    fn roll(&mut self, pins: i32) -> Result<(), BowlingError> {
        if self.finished() {
            return Err(BowlingError::GameFinished(format!(
                "Game already finished with {} frames",
                MAX_FRAMES
            )));
        }

        if pins <= STRIKE {
            self.update_frames(pins)?;
            Ok(self.update_score())
        } else {
            Err(BowlingError::InvalidRoll(format!(
                "Invalid roll with pins: {}",
                pins
            )))
        }
    }

    fn score(&self) -> i32 {
        self.current_score
    }

    fn finished(&self) -> bool {
        self.frames.len() == MAX_FRAMES && self.frames.last().unwrap().complete(true)
    }
}

#[cfg(test)]
mod test {
    use super::{Bowling, TenPinBowling};
    use crate::BowlingError;

    #[test]
    fn test_no_rolls_score_is_zero() {
        let bowling = TenPinBowling::new();
        assert_eq!(0, bowling.score());
    }

    #[test]
    fn test_single_roll_score() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(5).unwrap();
        assert_eq!(5, bowling.score());
    }

    #[test]
    fn test_frame_score() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(5).unwrap();
        bowling.roll(4).unwrap();
        assert_eq!(9, bowling.score());
    }

    #[test]
    fn test_frame_with_strike() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(10).unwrap();
        assert_eq!(10, bowling.score());
    }

    #[test]
    fn test_rolls_after_strike() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(10).unwrap();
        bowling.roll(5).unwrap();
        bowling.roll(4).unwrap();
        assert_eq!(28, bowling.score());
    }

    #[test]
    fn test_rolls_after_spare() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(4).unwrap();
        bowling.roll(6).unwrap();
        bowling.roll(5).unwrap();
        bowling.roll(3).unwrap();
        assert_eq!(23, bowling.score());
    }

    #[test]
    fn test_invalid_rolls() {
        let mut bowling = TenPinBowling::new();
        assert!(bowling.roll(4).is_ok());
        assert!(bowling.roll(8).is_err()); // now frame is 11 which is invalid
    }

    #[test]
    fn test_max_score() {
        let mut bowling = TenPinBowling::new();
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert!(bowling.roll(10).is_ok());
        assert_eq!(300, bowling.score());

        // No more play is allowed after the last frame
        assert_eq!(
            Err(BowlingError::GameFinished(format!(
                "Game already finished with 10 frames"
            ))),
            bowling.roll(4)
        );
    }

    fn get_score(bowling: &mut TenPinBowling, rolls: &[i32]) -> i32 {
        rolls.iter().for_each(|&r| bowling.roll(r).unwrap());
        bowling.score()
    }

    // Note: tested these values with https://www.bowlinggenius.com/ calculator
    #[test]
    fn test_scores() {
        assert_eq!(3, get_score(&mut TenPinBowling::new(), &[1, 1, 1]));
        assert_eq!(
            12,
            get_score(
                &mut TenPinBowling::new(),
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            )
        );
        assert_eq!(
            14,
            get_score(
                &mut TenPinBowling::new(),
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            )
        );

        // Just rolling 20 of 1s
        assert_eq!(
            20,
            get_score(
                &mut TenPinBowling::new(),
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
            )
        );

        // Just rolling 17 of 1s with a spare in last frame allowing one more roll (3)
        assert_eq!(
            31,
            get_score(
                &mut TenPinBowling::new(),
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 4, 6, 3]
            )
        );

        // Just rolling 17 of 1s with a spare in last frame allowing one more roll (3)
        assert_eq!(
            48,
            get_score(
                &mut TenPinBowling::new(),
                &[1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 10, 10, 10]
            )
        );
    }
}
