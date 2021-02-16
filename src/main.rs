#[macro_use]
extern crate rusqlite;

use structopt::StructOpt;

mod database;
mod opt;
mod task;

use database::*;
use opt::Opt;
use std::cmp::Ord;

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Add {
            priority,
            list,
            data,
        } => {
            let task = data.join(" ");
            match list {
                Some(list) => new_task(task, priority.unwrap_or(0), list),
                None => new_task_current(task, priority.unwrap_or(0)),
            }
        }
        Opt::Clean { list } => {
            let count = match list {
                Some(list) => clean(list),
                None => clean_current(),
            };
            println!("Removed {} items", count.unwrap());
        }
        Opt::Complete { num } => complete(num),
        Opt::Current => {
            let (_, name) = get_current_list();
            println!("Current list is : {}", name);
        }
        Opt::List { list, order } => {
            let mut tasks = match list {
                Some(list) => get_tasks(list),
                None => get_current_tasks(),
            };
            match order.as_deref() {
                Some("num") => tasks.sort_by(|task, other| task.num.cmp(&other.num)),
                _ => tasks.sort_by(|task, other| {
                    other
                        .priority
                        .partial_cmp(&task.priority)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }),
            }
            for task in tasks {
                println!("{}", task);
            }
        }
        Opt::Edit { list, num, data } => {
            let string = data.join(" ");
            match list {
                Some(list) => update_desc_list(num, string, list),
                None => update_desc(num, string),
            }
        }
        Opt::Remove { list_mode, value } => {
            if list_mode {
                remove_list(value);
            } else {
                match value.parse() {
                    Ok(num) => remove_task(num),
                    Err(_) => panic!("Could not parse num"),
                }
            }
        }
        Opt::Swap {
            list,
            num_one,
            num_two,
        } => match list {
            Some(name) => swap(num_one, num_two, name),
            None => swap_current(num_one, num_two),
        },
        Opt::Switch { list } => switch_list(list),
        Opt::Update { list } => {
            match list {
                Some(name) => update_nums(name).unwrap(),
                None => update_current_nums().unwrap(),
            };
        }
    }
}
