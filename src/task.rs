use colored::Colorize;
use std::fmt;

const BOX_CHECKED: char = '☑';
const BOX_UNCHECKED: char = '☐';

#[derive(Debug, PartialOrd, PartialEq)]
pub enum Priority {
    LOW,
    MED,
    HIGH,
}

impl Priority {
    pub fn new<T: Into<i32>>(val: T) -> Priority {
        match val.into() {
            0 => Priority::LOW,
            1 => Priority::MED,
            2 => Priority::HIGH,
            _ => panic!("unknown priority"),
        }
    }
}

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub num: u32,
    pub data: String,
    pub complete: bool,
    pub priority: Priority,
    pub list: String,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out: String;
        if self.complete {
            out = format!("{} {:03}: {}", BOX_CHECKED, self.num, self.data);
        } else {
            out = format!("{} {:03}: {}", BOX_UNCHECKED, self.num, self.data);
        }
        match self.priority {
            Priority::LOW => write!(f, "{}", out.cyan()),
            Priority::MED => write!(f, "{}", out.yellow()),
            Priority::HIGH => write!(f, "{}", out.red()),
        }
    }
}
