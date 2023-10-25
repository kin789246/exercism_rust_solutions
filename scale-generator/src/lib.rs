// You should change this.
//
// Depending on your implementation, there are a variety of potential errors
// which might occur. They aren't checked by the test suite in order to
// allow the greatest freedom of implementation, but real libraries should
// provide useful, descriptive errors so that downstream code can react
// appropriately.
//
// One common idiom is to define an Error enum which wraps all potential
// errors. Another common idiom is to use a helper type such as failure::Error
// which does more or less the same thing but automatically.
#[derive(Debug)]
pub struct Error {
    msg: String
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl Error {
    pub fn msg(m: &str) -> Self {
        Self { msg: m.to_string() }
    }
}

pub struct Scale {
    scale: Vec<String>,
}

impl Scale {
    fn get_pitches(tonic: &str) -> Option<Vec<&str>> {
        let sharps: Vec<&str> = vec!["G", "D", "A", "E", "B", "C", "F#", "a", "e", "b", "f#", "c#", "g#", "d#"];
        let flats: Vec<&str> = vec!["F", "Bb", "Eb", "Ab", "Db", "Gb", "d", "g", "c", "f", "bb", "eb"];
        let pitches: Vec<&str>;
        if sharps.iter().any(|&s| s == tonic) {
            pitches = vec!["A", "A#", "B", "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#"];
        }
        else if flats.iter().any(|&s| s == tonic) {
            pitches = vec!["A", "Bb", "B", "C", "Db", "D", "Eb", "E", "F", "Gb", "G", "Ab"];
        }
        else {
            return None
        }
        Some(pitches)
    }
    
    fn find_pitch(tonic: &str, pitches: &Vec<&str>) -> Option<usize> {
        for (i, &v) in pitches.iter().enumerate() {
            if v.to_lowercase() == tonic.to_lowercase() {
                return Some(i);
            }
        }
        None
    }

    pub fn new(tonic: &str, intervals: &str) -> Result<Scale, Error> {
        if let Some(pitches) = Self::get_pitches(tonic) {
            if let Some(mut i) = Self::find_pitch(tonic, &pitches) {
                let mut ans: Vec<String> = Vec::new();
                ans.push(pitches[i].to_string());
                for c in intervals.chars() {
                    if c == 'm' {
                        i = (i+1) % pitches.len();
                    }
                    else if c == 'M' {
                        i = (i+2) % pitches.len();
                    }
                    else if c == 'A' {
                        i = (i+3) % pitches.len();
                    }
                    else {
                        return Err(Error::msg("intervals is incorrect"));
                    }
                    ans.push(pitches[i].to_string());
                }
                return Ok(Scale{ scale: ans });
            }
            else {
                return Err(Error::msg("can't find pitch"));
            }
        }
        else {
            return Err(Error::msg("can't get pitches"));
        }
    }

    pub fn chromatic(tonic: &str) -> Result<Scale, Error> {
        if let Some(pitches) = Self::get_pitches(tonic) {
            if let Some(mut i) = Self::find_pitch(tonic, &pitches) {
                let mut ans: Vec<String> = Vec::new();
                ans.push(pitches[i].to_string());
                for _j in 0..12 {
                    i = (i+1) % pitches.len();
                    ans.push(pitches[i].to_string());
                }
                return Ok(Scale{ scale: ans });
            }
            else {
                return Err(Error::msg("can't find pitch"));
            }
        }
        else {
            return Err(Error::msg("can't get pitches"));
        }
    }

    pub fn enumerate(&self) -> Vec<String> {
        self.scale.to_owned()
    }
    // 1) A – B = whole step
    // 2) B – C = half step
    // 3) C – D = whole step
    // 4) D – E= whole step
    // 5) E – F= half step
    // 6) F – G= whole step
    // 7) G – A= whole step
}
