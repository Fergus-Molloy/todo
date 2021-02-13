use std::fmt;
mod database;
mod task;

fn main() {
    database::get_all_tasks();
}
