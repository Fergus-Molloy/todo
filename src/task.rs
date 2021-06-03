use colored::Colorize;
use std::fmt;

const BOX_CHECKED: char = '☑';
const BOX_UNCHECKED: char = '☐';

/// Priority of the task
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Priority {
    LOW,
    MED,
    HIGH,
}

impl Priority {
    /// Create new priority from an int
    pub fn new<T: Into<i32>>(val: T) -> Priority {
        match val.into() {
            0 => Priority::LOW,
            1 => Priority::MED,
            2 => Priority::HIGH,
            _ => panic!("unknown priority"),
        }
    }
}

/// Struct to represent a task
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
    /// Print a task
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = if self.complete {
            format!("{} {:03}: {}", BOX_CHECKED, self.num, self.data)
        } else {
            format!("{} {:03}: {}", BOX_UNCHECKED, self.num, self.data)
        };
        match self.priority {
            Priority::LOW => write!(f, "{}", out.cyan()),
            Priority::MED => write!(f, "{}", out.yellow()),
            Priority::HIGH => write!(f, "{}", out.red()),
        }
    }
}
