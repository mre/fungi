
/// disclaimer: this is gonna be ugly.

#[derive(Debug, PartialEq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
    Boom,
}

pub struct BowlingGame {
    frames: Vec<(Option<u16>, Option<u16>)>,
    last: (Option<u16>, Option<u16>),
}

impl BowlingGame {
    pub fn new() -> Self {
        return Self {
            frames: Vec::with_capacity(10),
            last: (None, None),
        };
    }

    // "Record that {} pins have been scored"
    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        // more than 10 frames and and one throw the game is over
        if self.frames.len() > 10 && self.last.0.is_some() {
            return Err(Error::GameComplete);
        }

        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft);
        }

        // checking the 10th frame
        if self.frames.len() == 10 {
            // if was a strike, add a throw
            if is_strike(self.frames.last().unwrap()) {
                if self.last.0.is_none() {
                    self.last.0 = Some(pins);
                    if pins == 10 {
                        self.frames.push(self.last);
                        self.last = (None, None);
                    }
                    return Ok(());
                }

                if self.last.0.unwrap_or(0) + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }

                self.last.1 = Some(pins);
                // close the frame
                self.frames.push(self.last);
                self.last = (None, None);
                return Ok(());
            }

            // if was a spare, add the single throw
            if is_spare(self.frames.last().unwrap()) {
                self.last.0 = Some(pins);
                self.frames.push(self.last);
                self.last = (None, None);
                return Ok(());
            }

            // if was not a strike, error
            if !is_strike(self.frames.last().unwrap()) {
                return Err(Error::GameComplete);
            }

            // if was not a spare, error
            if !is_spare(self.frames.last().unwrap()) {
                return Err(Error::GameComplete);
            }
        }

        if self.frames.len() == 11 && !is_strike(self.frames.last().unwrap()) {
            return Err(Error::GameComplete);
        }

        // look at the last throw
        match self.last {
            // this is a new frame, first shot
            (None, None) => {
                // account the first attempt
                self.last.0 = Some(pins);
                if pins == 10 {
                    self.frames.push(self.last);
                    self.last = (None, None);
                }

                return Ok(());
            }

            // this is an existing frame, second shot
            (Some(_), None) => {
                // validation, there are at most 10 pins per frame
                if self.last.0.unwrap_or(0) + pins > 10 {
                    return Err(Error::NotEnoughPinsLeft);
                }

                // drop the pins
                self.last.1 = Some(pins);
                // close the frame
                self.frames.push(self.last);
                self.last = (None, None);

                // exit the roll
                return Ok(());
            }

            // this should not happen
            // (Some(_), Some(_)) => return Err(Error::Boom),

            // this should not happen
            _ => return Err(Error::Boom),
        }
    }

    pub fn score(&self) -> Option<u16> {
        let mut fs: Vec<(Option<u16>, Option<u16>)> = self.frames.clone();
        if self.last.0.is_some() {
            fs.push(self.last);
        }

        // game is not over
        if fs.len() < 10 {
            return None;
        }

        // the score of the game
        let mut points: u16 = 0;

        // looping the frames
        for i in 0..fs.len() {
            let f = fs[i];

            if i > 9 {
                // The tenth frame in the game is a special case. If
                // someone throws a strike or a spare then they get a
                // fill ball. Fill balls exist to calculate the total of
                // the 10th frame. Scoring a strike or spare on the fill
                // ball does not give the player more fill balls. The
                // total value of the 10th frame is the total number of
                // pins knocked down.
                //
                // For a tenth frame of X1/ (strike and a spare), the
                // total value is 20.
                //
                // For a tenth frame of XXX (three strikes), the total
                // value is 30.
                continue;
            }

            points = points + f.0.unwrap_or(0) + f.1.unwrap_or(0);

            // A spare is where all ten pins are knocked down by the
            // second throw. The total value of a spare is 10 plus
            // the number of pins knocked down in their next throw.
            if is_spare(&f) {
                let next: (Option<u16>, Option<u16>) = if i + 1 < fs.len() {
                    fs[i + 1]
                } else {
                    self.last
                };
                if next.0.is_none() {
                    return None;
                }
                points = points + next.0.unwrap();
            }

            // A strike is where all ten pins are knocked down by
            // the first throw. The total value of a strike is 10
            // plus the number of pins knocked down in the next two
            // throws. If a strike is immediately followed by a
            // second strike, then the value of the first strike
            // cannot be determined until the ball is thrown one
            // more time.
            if is_strike(&f) {
                let mut next: (Option<u16>, Option<u16>) = if i + 1 < fs.len() {
                    fs[i + 1]
                } else {
                    self.last
                };
                if next.0.is_none() {
                    return None;
                }
                points = points + next.0.unwrap();
                if next.1.is_some() {
                    points = points + next.1.unwrap();
                } else {
                    next = if i + 2 < fs.len() {
                        fs[i + 2]
                    } else {
                        self.last
                    };
                    if next.0.is_none() {
                        return None;
                    }
                    points = points + next.0.unwrap();
                }
            }
        }
        return Some(points);
    }
}

fn is_spare(frame: &(Option<u16>, Option<u16>)) -> bool {
    if frame.0.is_none() || frame.1.is_none() {
        return false;
    };
    return !is_strike(frame) && frame.0.unwrap() + frame.1.unwrap() == 10;
}

fn is_strike(frame: &(Option<u16>, Option<u16>)) -> bool {
    return frame.0.is_some() && frame.0.unwrap() == 10;
}
