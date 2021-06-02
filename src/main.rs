#[macro_use]
extern crate rusqlite;

use structopt::StructOpt;

mod database;
mod opt;
mod task;

use crate::task::Task;
use opt::Cmd;
use opt::Opt;

fn main() {
    let opt = Opt::from_args();
    match opt {
        Opt::Add {
            cmd,
            priority,
            list,
            data,
        } => match cmd {
            // change to act like remove (use cmd to specify adding a list and adding task to
            // specific list)
            Some(cmd) => {
                // using if let because it's much more neat than a single arm match
                #[allow(irrefutable_let_patterns)]
                if let Cmd::List { list_name } = cmd {
                    match database::add::create_list(&list_name) {
                        Ok(_) => println!("Created list {}", list_name),
                        Err(e) => {
                            eprintln!(
                                "Could not create list {}\n An error occurred: {}",
                                list_name, e
                            );
                            std::process::exit(1);
                        }
                    }
                }
            }
            None => database::add::new_task(data.join(" "), priority.unwrap_or(0), list),
        },
        Opt::Clean { list } => {
            let count = database::clean::clean(list).unwrap();
            if count > 1 {
                println!("Removed {} items", count);
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
        Opt::Remove { list, value } => match list {
            Some(cmd) => {
                // using if let because it's much more neat than a single arm match
                #[allow(irrefutable_let_patterns)]
                if let Cmd::List { list_name } = cmd {
                    database::remove::remove_list(list_name)
                }
            }
            None => match value {
                Some(num) => database::remove::remove_task(num, None),
                None => {
                    eprintln!("No list or value given, exiting");
                    std::process::exit(1);
                }
            },
        },
        Opt::Swap {
            list,
            num_one,
            num_two,
        } => database::swap::swap(num_one, num_two, list),
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

fn order_tasks(task_list: &mut Vec<Task>, order: Option<String>) {
    match order.as_deref() {
        // order tasks
        Some("num") => task_list.sort_by(|task, other| task.num.cmp(&other.num)),
        _ => task_list.sort_by(|task, other| {
            other
                .priority
                .partial_cmp(&task.priority)
                .unwrap_or(std::cmp::Ordering::Less)
        }),
    }
}
