use core::fmt;

#[derive(Debug)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl std::cmp::PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hour==other.hour && self.minute==other.minute
    }
}

impl Clock {
    fn on_computing(hours: i32, minutes: i32) -> (i32, i32) {
        let carry = match minutes {
            x if x%60 < 0 => x / 60 - 1,
            _ => minutes / 60,
        }; 
        let h = hours + carry;
        let h = match h {
            x if x%24 < 0 => 24 + h%24,
            _ => h%24,
        };
        let m = match minutes {
            x if x%60 < 0 => 60 + x%60,
            _ => minutes%60,
        };
        (h, m)
    }

    pub fn new(hours: i32, minutes: i32) -> Self {
        //unimplemented!("Construct a new Clock from {hours} hours and {minutes} minutes");
        let (h, m) = Clock::on_computing(hours, minutes);
        Clock { hour: h, minute: m }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        //unimplemented!("Add {minutes} minutes to existing Clock time");
        let (h, m) = Clock::on_computing(self.hour, self.minute + minutes);
        Clock { hour: h, minute: m }
    }
}
