use std::{cell::RefCell, collections::HashSet};
use rand::{thread_rng, Rng};

thread_local!(static USED_NAMES: RefCell<HashSet<String>> = 
    RefCell::new(HashSet::new()));
pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot { name: Robot::get_unique_name() }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        USED_NAMES.with(|cell| {
            cell.borrow_mut().remove(self.name())
        });
        self.name = Robot::get_unique_name();
    }
    fn get_unique_name() -> String {
        USED_NAMES.with(|cell| {
            let mut used_names = cell.borrow_mut();
            loop {
                let name = Robot::gen_name();
                if used_names.insert(name.clone()) {
                    return name
                }
            }
        })
    }
    fn gen_name() -> String {
        let mut rng = thread_rng();
        format!("{}{}{:03}", rng.gen_range(b'A'..=b'Z') as char,
                                    rng.gen_range(b'A'..=b'Z') as char,
                                    rng.gen_range(0..1000)
                )
    }
}
