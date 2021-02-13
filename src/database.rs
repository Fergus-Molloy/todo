use crate::task::{Priority, Task};
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::collections::HashMap;
use std::path::PathBuf;

fn db_get() -> PathBuf {
    let mut todo = dirs::home_dir().unwrap();
    todo.push(".todo.db");
    if !todo.exists() {
        // create db
        std::fs::File::create(&todo).unwrap();
    }
    assert!(todo.exists() && todo.is_file());
    todo
}

fn connect() -> Result<Connection> {
    let todo = db_get();
    Connection::open(todo)
}

pub fn get_all_tasks() -> Vec<Task> {
    let con = connect().unwrap();
    let get = r"
    SELECT tasks.*, lists.name FROM task_to_list
    INNER JOIN tasks ON tasks.id=task_to_list.task
    INNER JOIN lists ON lists.id=task_to_list.list
    ";
    let mut stmt = con.prepare(get).unwrap();
    let stmt_iter = stmt
        .query_map(NO_PARAMS, |row| {
            Ok(Task {
                id: row.get(0)?,
                num: row.get(1)?,
                complete: {
                    let res: i32 = row.get(2).unwrap();
                    res == 1
                },
                priority: {
                    let res: i32 = row.get(3).unwrap();
                    Priority::new(res)
                },
                data: row.get(4)?,
                list: row.get(5)?,
            })
        })
        .unwrap();

    for task in stmt_iter {
        println!("{:?}", task.unwrap());
    }

    let tasks: Vec<Task> = Vec::new();
    tasks
}

fn get_next_num() -> String {
    // query database for next available num
    todo!();
}

pub fn add<T: Into<i32>>(data: String, priority: T) {
    let num = get_next_num();
    let priority = Priority::new(priority);
    let complete = false;
    // insert  task here to generate id
}

pub fn complete(num: u32) {
    // find and update task to be complete
    todo!()
}

fn update_nums() {
    // update nums of all items so there are no gaps and high priority tasks have ids < lower
    // priority tasks
    todo!()
}

pub fn clean() {
    // find completed tasks and remove them
    update_nums();
    todo!()
}

pub fn new_list(name: String) {
    // create a new table with the given name for a new task list
    todo!()
}

pub fn switch_list(name: String) {
    // change default list to given list
    //if(!list.exists(){
    //    confirm_new_list(name);
    //    new_list(name);
    //} else {
    //    update_list(name);
    //}
    todo!()
}
