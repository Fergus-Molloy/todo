#[macro_use]
extern crate rusqlite;
use structopt::StructOpt;
mod database;
use database::*;
mod opt;
use opt::Opt;
mod task;
fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
    match opt {
        Opt::List { list } => {
            let tasks = match list {
                Some(list) => get_tasks(list),
                None => get_current_tasks(),
            };
            for task in tasks {
                println!("{}", task);
            }
        }
        Opt::Clean { list } => {
            let count = match list {
                Some(list) => clean(list),
                None => clean_current(),
            };
            println!("Removed {} items", count.unwrap());
        }
        Opt::Current => {
            let (_, name) = get_current_list();
            println!("Current list is : {}", name);
        }
        Opt::Complete { num } => complete(num),
        Opt::Add {
            priority,
            list,
            data,
        } => {
            let mut task = String::new();
            for word in data {
                task.push_str(&format!("{} ", word)[..]);
            }
            match list {
                Some(list) => new_task(task, priority.unwrap_or(0), list),
                None => new_task_current(task, priority.unwrap_or(0)),
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
        _ => todo!(),
    }
}
