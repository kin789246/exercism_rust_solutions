// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    years: f64,
}

//const DAYS_EARTH:f64 = 365.25;
const SECS_EARTH:f64 = 31557600.0;
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { years: s as f64 / SECS_EARTH }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.years
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_Planet {
    ($($t:ty| $l:expr),+) => {
        $(impl Planet for $t {
                fn years_during(d: &Duration) -> f64 {
                    d.years / $l
                }
        })*
    }
}

const MERCURY:f64 = 0.2408467;
const VENUS:f64 = 0.61519726;
const EARTH:f64 = 1.0;
const MARS:f64 = 1.8808158;
const JUPITER:f64 = 11.862615;
const SATURN:f64 = 29.447498;
const URANUS:f64 = 84.016846;
const NEPTUNE:f64 = 164.79132;

impl_Planet!(Mercury | MERCURY, 
            Venus | VENUS,
            Earth | EARTH,
            Mars | MARS,
            Jupiter | JUPITER,
            Saturn | SATURN,
            Uranus | URANUS,
            Neptune | NEPTUNE);

/*
impl Planet for Mercury {}
impl Planet for Venus {}
impl Planet for Earth {}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
*/