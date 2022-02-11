pub trait Bowling {
    fn roll(&mut self, pins: i32);
    fn score(&self) -> i32;
}

static STRIKE: i32 = 10;

#[derive(Debug)]
struct Frame {
    rolls: Vec<i32>,
}

impl Frame {
    fn with_roll(pins: i32) -> Self {
        Self { rolls: vec![pins] }
    }

    fn roll(&mut self, pins: i32) {
        self.rolls.push(pins);
    }

    fn complete(&self) -> bool {
        self.strike() || self.rolls.len() >= 2
    }

    fn strike(&self) -> bool {
        self.rolls.len() == 1 && self.rolls[0] == STRIKE
    }

    fn spare(&self) -> bool {
        self.rolls.len() == 2 && self.sum() == STRIKE
    }

    fn sum(&self) -> i32 {
        self.rolls.iter().sum()
    }

    fn last_roll(&self) -> i32 {
        *self.rolls.last().unwrap()
    }
}

#[derive(Debug)]
pub struct TenPinBowling {
    frames: Vec<Frame>,
    current_score: i32,
}

impl TenPinBowling {
    fn update_frames(&mut self, pins: i32) {
        match self.current_frame_mut() {
            None => {
                self.add_new_frame(pins);
            }
            Some(frame) => {
                if frame.complete() {
                    self.add_new_frame(pins);
                } else {
                    frame.roll(pins);
                }
            }
        }
    }

    fn add_new_frame(&mut self, pins: i32) {
        self.frames.push(Frame::with_roll(pins));
    }

    fn update_score(&mut self) {
        match self.current_frame() {
            None => {}
            Some(current_frame) => match self.last_frame() {
                None => {
                    self.current_score += current_frame.last_roll();
                }
                Some(last_frame) => {
                    if last_frame.strike() {
                        self.current_score += current_frame.last_roll() * 2;
                    } else if last_frame.spare() {
                        if current_frame.complete() {
                            self.current_score += current_frame.last_roll()
                        } else {
                            self.current_score += current_frame.last_roll() * 2;
                        }
                    } else {
                        self.current_score += current_frame.last_roll();
                    }
                }
            },
        }
    }

    fn last_frame(&self) -> Option<&Frame> {
        let n = self.frames.len();
        if n >= 2 {
            self.frames.get(n - 2)
        } else {
            None
        }
    }

    fn current_frame_mut(&mut self) -> Option<&mut Frame> {
        self.frames.last_mut()
    }

    fn current_frame(&self) -> Option<&Frame> {
        self.frames.last()
    }

    pub fn new() -> Self {
        Self {
            frames: vec![],
            current_score: 0,
        }
    }
}

impl Bowling for TenPinBowling {
    fn roll(&mut self, pins: i32) {
        self.update_frames(pins);
        self.update_score();
    }

    fn score(&self) -> i32 {
        self.current_score
    }
}

#[cfg(test)]
mod test {
    use super::{Bowling, TenPinBowling};

    #[test]
    fn test_no_rolls_score_is_zero() {
        let bowling = TenPinBowling::new();
        assert_eq!(0, bowling.score());
    }

    #[test]
    fn test_single_roll_score() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(5);
        assert_eq!(5, bowling.score());
    }

    #[test]
    fn test_frame_score() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(5);
        bowling.roll(4);
        assert_eq!(9, bowling.score());
    }

    #[test]
    fn test_frame_with_strike() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(10);
        assert_eq!(10, bowling.score());
    }

    #[test]
    fn test_rolls_after_strike() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(10);
        bowling.roll(5);
        bowling.roll(4);
        assert_eq!(28, bowling.score());
    }

    #[test]
    fn test_rolls_after_spare() {
        let mut bowling = TenPinBowling::new();
        bowling.roll(4);
        bowling.roll(6);
        bowling.roll(5);
        bowling.roll(3);
        assert_eq!(23, bowling.score());
    }
}
