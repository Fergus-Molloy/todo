#[macro_use]
extern crate rusqlite;

use structopt::StructOpt;

mod database;
mod opt;
mod task;

use database::*;
use opt::Cmd;
use opt::Opt;
use std::cmp::Ord;

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", opt);
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
                    match create_list(&list_name) {
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
            None => new_task(data.join(" "), priority.unwrap_or(0), list),
        },
        Opt::Clean { list } => {
            let count = clean(list);
            println!("Removed {} items", count.unwrap());
        }
        Opt::Complete { num } => match complete(num) {
            Ok(_) => println!("Completed {:03}", num),
            Err(e) => panic!("could not update {}:\n{}", num, e),
        },
        Opt::Current => {
            // change this to lists and list to tasks??
            println!("Current list is : {}", get_current_list_name())
        }
        Opt::List { list, order } => {
            let mut tasks = get_tasks(list); // get tasks
            match order.as_deref() {
                // order tasks
                Some("num") => tasks.sort_by(|task, other| task.num.cmp(&other.num)),
                _ => tasks.sort_by(|task, other| {
                    other
                        .priority
                        .partial_cmp(&task.priority)
                        .unwrap_or(std::cmp::Ordering::Equal)
                }),
            }
            // print tasks
            for task in tasks {
                println!("{}", task);
            }
        }
        Opt::Edit { list, num, data } => match update_desc(num, data.join(" "), list) {
            Ok(_) => println!("Sucessfully updated description of task {:03}", num),
            Err(e) => eprintln!("Could not update {}\nReason: {}", num, e),
        },
        Opt::Remove { list, value } => match list {
            Some(cmd) => {
                // using if let because it's much more neat than a single arm match
                #[allow(irrefutable_let_patterns)]
                if let Cmd::List { list_name } = cmd {
                    remove_list(list_name)
                }
            }
            None => match value {
                Some(num) => remove_task(num, None),
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
        } => swap(num_one, num_two, list),
        Opt::Switch { list } => match switch_list(&list) {
            Ok(_) => println!("Set current list to {}", list),
            Err(e) => eprintln!("Could not update!\nReason: {}", e),
        },
        Opt::Update { list } => println!("Updated {} items", update_nums(list).unwrap()),
    }
}
