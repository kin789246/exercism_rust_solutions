#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
#[derive(Debug)]
pub struct BowlingGame{
    frames: Vec<Vec<u16>>,
}

impl BowlingGame {
    pub fn new() -> Self {
        Self { frames: vec![vec![]], }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        //unimplemented!("Record that {pins} pins have been scored");
        if pins > 10 { return Err(Error::NotEnoughPinsLeft) }
        let n = self.frames.len() - 1;
        if self.frames.len() < 10  {
            match self.frames[n].len() {
                0 => self.frames[n].push(pins),
                1 => {
                    if self.frames[n][0] == 10 {
                        self.frames.push(vec![pins]);
                    }
                    else {
                        if pins + self.frames[n].iter().sum::<u16>() > 10 { 
                            return Err(Error::NotEnoughPinsLeft) 
                        }
                        self.frames[n].push(pins);
                    }
                },
                2 => self.frames.push(vec![pins]),
                _ => return Err(Error::GameComplete)
            }
        }
        else if self.frames.len() == 10 {
            match self.frames[n].len() {
                0 | 1 => self.frames[n].push(pins),
                2 => {
                    if self.frames[n][0] == 10 {
                        if (pins + self.frames[n][1] > 10) && self.frames[n][1] != 10 { 
                            return Err(Error::NotEnoughPinsLeft) 
                        }
                        self.frames[n].push(pins);
                    }
                    else {
                        if self.frames[n].iter().sum::<u16>() < 10 { 
                            return Err(Error::GameComplete) 
                        }
                        self.frames[n].push(pins);
                    }
                },
                _ => return Err(Error::GameComplete)
            }
        }
        else {
            return Err(Error::GameComplete)
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        //unimplemented!("Return the score if the game is complete, or None if not.");
        if self.frames.len() < 10 { return None }
        if self.frames.len() == 10 {
            if self.frames[9].len() == 2 && self.frames[9].iter().sum::<u16>() == 10 {
                return None
            }
            if self.frames[9][0] == 10 && self.frames[9].len() < 3 {
                return None
            }
        }
        let mut sum: u16 = 0;
        let mut cur: usize = 0;
        while cur < self.frames.len() {
            if cur < 9 {
                if self.frames[cur][0] == 10 {
                    if self.frames[cur+1].len() == 1 {
                        sum += self.frames[cur+2][0] + 10;
                    }
                    else {
                        sum += self.frames[cur+1][0] + self.frames[cur+1][1];
                    }
                }
                if self.frames[cur].len() == 2 && self.frames[cur].iter().sum::<u16>() == 10 {
                    sum += self.frames[cur+1][0];
                }
            }
            else {

            }
            sum += self.frames[cur].iter().sum::<u16>();
            cur += 1;
        }
        Some(sum)
    }
}