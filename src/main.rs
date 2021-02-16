#[macro_use]
extern crate rusqlite;
use std::cmp::Ord;
use structopt::StructOpt;
mod database;
use database::*;
mod opt;
use opt::Opt;
mod task;
fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Add {
            priority,
            list,
            data,
        } => {
            let task = string_from_vec(data);
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
            let order = order.unwrap_or("priority".to_string());
            match &order[..] {
                "num" => tasks.sort_by(|task, other| task.num.cmp(&other.num)),
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
            let string = string_from_vec(data);
            match list {
                Some(list) => update_desc_list(num, string, list),
                None => update_desc(num, string),
            }
        }
        Opt::Remove { list_mode, value } => {
            if list_mode {
                remove_list(value);
            } else {
                match value.parse::<i32>() {
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

fn string_from_vec(vec: Vec<String>) -> String {
    let mut string = String::new();
    for word in vec {
        string.push_str(&format!("{} ", word)[..]);
    }
    string
}
