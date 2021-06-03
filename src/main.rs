#![warn(missing_docs)]
//! This crate aims to implement a simple todo list command line interface
#[macro_use]
extern crate rusqlite;

use structopt::StructOpt;

mod database;
mod opt;
mod task;

use crate::task::Task;
use opt::{Cmd, Opt, RCmd};

/// Main function handles the CLI using structopt
///
/// You can read about each option in [Opt]
fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Add { cmd } => match cmd {
            Cmd::List { list_name } => match database::add::create_list(list_name.clone()) {
                Ok(_) => println!("Created list {}", list_name),
                Err(e) => {
                    eprintln!(
                        "Could not create list {}\n An error occurred: {}",
                        list_name, e
                    );
                    std::process::exit(1);
                }
            },
            Cmd::Task {
                priority,
                list,
                data,
            } => {
                println!("Adding task");
                match database::add::new_task(data.join(" "), priority.unwrap_or(0), list) {
                    Ok(_) => println!(
                        "Added task to list {}",
                        database::database::dynamic_list_name(&list)
                    ),
                    Err(e) => {
                        eprintln!("Could not add task to list: {}", e);
                        std::process::exit(1);
                    }
                }
            }
        },
        Opt::Clean { list } => {
            let count = database::clean::clean(list).unwrap();
            if count >= 1 {
                println!(
                    "Removed {} task{}",
                    count,
                    if count == 1 { "" } else { "s" }
                );
            } else {
                println!("No tasks cleaned");
            }
        }
        Opt::Complete { num, list } => match database::complete::complete(num, list) {
            Ok(updated) => {
                if updated == 1 {
                    println!("Completed {:03}", num)
                } else if updated > 1 {
                    eprintln!("Updated {} rows, something has gone wrong!", updated);
                } else {
                    println!("Task could not be completed (it might not exist)");
                }
            }
            Err(e) => panic!("could not update {}:\n{}", num, e),
        },
        Opt::Lists => {
            println!("Lists:");
            match database::lists::get_all_list_names() {
                Err(e) => eprintln!("Could not get lists: {}", e),
                Ok(list) => {
                    let lines: Vec<String> = list
                        .iter()
                        .map(|item| format!("{}{}", if item.1 { "* " } else { "" }, item.0))
                        .collect();
                    for line in lines {
                        println!("{}", line);
                    }
                }
            }
        }
        Opt::Tasks { list, order } => {
            let mut tasks = database::tasks::get_tasks(list.clone()); // get tasks
            let list_id = match database::database::list_exists(list) {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("An error occured getting the list name: {}", e);
                    std::process::exit(1);
                }
            };

            println!(
                "{}:",
                database::database::get_list_name(list_id)
                    .expect("An error occured getting the list name")
            );
            if tasks.len() > 1 {
                order_tasks(&mut tasks, order);
                for task in tasks {
                    println!("{}", task);
                }
            } else if tasks.len() == 1 {
                println!("{}", tasks[0]);
            } else {
                println!("No tasks here");
            }
        }
        Opt::Edit { list, num, data } => {
            match database::edit::update_desc(num, data.join(" "), list) {
                Ok(_) => println!("Sucessfully updated description of task {:03}", num),
                Err(e) => eprintln!("Could not update {}\nReason: {}", num, e),
            }
        }
        Opt::Remove { cmd } => match cmd {
            RCmd::List { list_name } => database::remove::remove_list(list_name),
            RCmd::Task { num, list } => database::remove::remove_task(num, list),
        },
        Opt::Swap {
            list,
            num_one,
            num_two,
        } => match database::swap::swap(num_one, num_two, list) {
            Ok(_) => println!("Swapped {} and {}", num_one, num_two),
            Err(e) => {
                eprintln!("Could not swap: {}", e);
                std::process::exit(1);
            }
        },
        Opt::Switch { list } => match database::switch::switch_list(list.clone()) {
            Ok(_) => println!("Set current list to {}", list),
            Err(e) => eprintln!("Could not update!\nReason: {}", e),
        },
        Opt::Update { list } => println!(
            "Updated {} items",
            database::update::update_nums(list).unwrap()
        ),
        Opt::Test => println!(
            "{:?}",
            database::database::list_exists(Some(String::from("test")))
        ),
    }
}

/// All implemented sorting methods. Defaults to Priority sorting
enum Sort {
    /// Sort by number assigned to task
    Num,
    /// Sort by Priority of the tasks
    Priority,
}

/// Function to parse the user's input into the sorting methods
fn parse_order(inp: Option<String>) -> Sort {
    match inp.as_deref() {
        Some(string) => match string {
            "num" => Sort::Num,
            _ => Sort::Priority,
        },
        None => Sort::Priority,
    }
}

/// Orders the task according to the user's input
fn order_tasks(task_list: &mut Vec<Task>, order: Option<String>) {
    match parse_order(order) {
        Sort::Num => task_list.sort_by(|task, other| task.num.cmp(&other.num)),
        Sort::Priority => task_list.sort_by(|task, other| {
            other
                .priority
                .partial_cmp(&task.priority)
                .unwrap_or(std::cmp::Ordering::Less)
        }),
    }
}
