#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

enum Frame {
    Open(u16, u16),
    Spare(u16, u16),
    Strike,
    Bonus(Option<u16>),
}

impl Frame {
    fn get_rolls(&self) -> Vec<u16> {
        match self {
            Self::Open(a, b) => vec![*a, *b],
            Self::Spare(a, b) => vec![*a, *b],
            Self::Strike => vec![10],
            Self::Bonus(Some(a)) => vec![*a],
            Self::Bonus(None) => vec![],
        }
    }
}

pub struct BowlingGame {
    prev_roll: Option<u16>, // prev roll in frame
    frames: Vec<Frame>,
    frames_left: i8,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            prev_roll: None,
            frames: Vec::with_capacity(12),
            frames_left: 10,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }
        if self.frames_left == 0 {
            return Err(Error::GameComplete);
        }

        if self.frames.len() < 10 {
            match (self.prev_roll, pins) {
                (None, 10) => {
                    self.frames.push(Frame::Strike);
                    self.frames_left -= 1;
                }
                (None, _) => {
                    self.prev_roll = Some(pins);
                }
                (Some(prev), _) => {
                    if prev + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                    if prev + pins == 10 {
                        self.frames.push(Frame::Spare(prev, pins));
                    } else {
                        self.frames.push(Frame::Open(prev, pins));
                    }
                    self.prev_roll = None;
                    self.frames_left -= 1;
                }
            }
            if self.frames.len() == 10 {
                match self.frames.last().unwrap() {
                    Frame::Strike => {
                        self.frames_left = 2;
                    }
                    Frame::Spare(_, _) => {
                        self.frames_left = 1;
                    }
                    _ => {
                        self.frames
                            .extend(vec![Frame::Bonus(None), Frame::Bonus(None)]);
                    }
                }
            }
        } else {
            // bonus throws
            self.frames.push(Frame::Bonus(Some(pins)));
            match (self.prev_roll, self.frames_left) {
                (None, 1) => {
                    self.frames.push(Frame::Bonus(None)); // pad for score()
                }
                (None, 2) => {
                    self.prev_roll = Some(pins);
                }
                (Some(prev), 1) => {
                    if prev != 10 && prev + pins > 10 {
                        return Err(Error::NotEnoughPinsLeft);
                    }
                }
                _ => unreachable!(),
            }
            self.frames_left -= 1;
        }

        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        if self.frames_left > 0 {
            None
        } else {
            let score = self
                .frames
                .windows(3)
                .map(|window| {
                    use Frame as F;
                    match window[..] {
                        [F::Strike, _, _] => {
                            10 + (window[1..]
                                .iter()
                                .flat_map(|f| f.get_rolls())
                                .take(2)
                                .sum::<u16>())
                        }
                        [F::Spare(_, _), _, _] => {
                            10 + window[1].get_rolls().iter().take(1).sum::<u16>()
                        }
                        [F::Open(a, b), _, _] => a + b,
                        _ => unreachable!(),
                    }
                })
                .sum();
            Some(score)
        }
    }
}
