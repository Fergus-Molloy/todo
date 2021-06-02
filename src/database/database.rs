use rusqlite::{Connection, Result, NO_PARAMS};
use std::fmt::Display;
use std::io;
use std::path::PathBuf;

// TODO: Create a function to check if list exists
//  - should take name of list
//  - return result (Ok(list_id) or Err(list doesn't exits))
// TODO: Create function to consume above error and ask user to create new list
//  - Should return result (Ok(list_id) or Err(rusqlite error))
//  - If user does not want to create list program should exit gracefully

// init
pub fn db_get() -> PathBuf {
    let mut todo = dirs::home_dir().unwrap();
    todo.push(".todo.db");
    if !todo.exists() {
        println!("creating new db at {:?}", todo);
        // create db
        std::fs::File::create(&todo).unwrap();
        let create_lists = r"
        CREATE TABLE 'lists' (
	'id'	INTEGER NOT NULL UNIQUE,
	'name'	TEXT NOT NULL UNIQUE,
	'current'	INTEGER,
	'MaxNum'	INTEGER,
	PRIMARY KEY('id' AUTOINCREMENT))";
        let create_tasks = r"
        CREATE TABLE 'tasks' (
	'id'	INTEGER NOT NULL UNIQUE,
	'num'	INTEGER NOT NULL,
	'complete'	INTEGER NOT NULL,
	'priority'	INTEGER NOT NULL,
	'data'	TEXT,
	PRIMARY KEY('id' AUTOINCREMENT));";
        let create_tasks_to_list = r"
        CREATE TABLE 'task_to_list' (
	'id'	INTEGER NOT NULL UNIQUE,
	'task'	INTEGER NOT NULL UNIQUE,
	'list'	INTEGER NOT NULL,
	FOREIGN KEY('task') REFERENCES 'tasks'('id') ON DELETE CASCADE,
	FOREIGN KEY('list') REFERENCES 'lists'('id') ON DELETE CASCADE,
	PRIMARY KEY('id' AUTOINCREMENT));";
        let mut path = dirs::home_dir().unwrap();
        path.push(".todo.db");
        let con = Connection::open(path);
        let con = match con {
            Ok(v) => v,
            Err(e) => panic!("this isn't going to be fun, {}", e),
        };
        con.execute(create_lists, NO_PARAMS).unwrap();
        con.execute(create_tasks, NO_PARAMS).unwrap();
        con.execute(create_tasks_to_list, NO_PARAMS).unwrap();
        con.execute(
            "INSERT INTO lists (name, current, MaxNum) VALUES('General', 1, 0)",
            NO_PARAMS,
        )
        .unwrap();
    }
    assert!(todo.exists() && todo.is_file());
    todo
}

// init
pub fn connect() -> Result<Connection> {
    let todo = db_get();
    Connection::open(todo)
}

// fetch
pub fn get_list_id(name: &String) -> Result<i32> {
    let con = connect().unwrap();
    let get = r"
    SELECT id FROM lists
    WHERE name==?
    ";
    let result = con.query_row(get, params![name], |row| Ok(row.get(0)?))?;
    Ok(result)
}

// fetch
pub fn get_current_list_name() -> String {
    let con = connect().unwrap();
    let get = "SELECT name FROM lists WHERE current==1";
    let mut stmt = con.prepare(get).unwrap();
    let res = stmt.query_row(NO_PARAMS, |row| {
        let name: String = row.get(0)?;
        Ok(name)
    });
    res.unwrap()
}

pub fn user_agreement<S: Display>(phrase: S) -> bool {
    let accept_phrases: [&str; 4] = ["y", "yes", "yeah", "yy"];
    println!("{}", phrase);
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("could not read input");
    accept_phrases.iter().any(|&x| x == inp)
}

// update
pub fn _update_nums(list: Option<String>) -> Result<usize> {
    let list = list.unwrap_or(get_current_list_name());
    let sql = r"
    SELECT t.id, t.priority, t.num FROM tasks AS t
    INNER JOIN task_to_list ON t.id==task_to_list.task
    INNER JOIN lists ON task_to_list.list==lists.id
    WHERE lists.name==?";
    let update = "UPDATE tasks SET num=? WHERE id==?";
    let con = connect().unwrap();
    let mut stmt = con.prepare(sql).unwrap();
    let iter = stmt
        .query_map(params![list], |row| {
            let id: i32 = row.get(0)?;
            let p: i32 = row.get(1)?;
            let num: i32 = row.get(2)?;
            println!("found task with id: {}", id);
            Ok((id, p, num))
        })
        .unwrap();
    let mut tasks = Vec::new();
    for row in iter {
        match row {
            Ok(v) => tasks.push(v),
            Err(e) => panic!("Something went wrong: {}", e),
        }
    }
    tasks.sort_by(|task, other| other.1.cmp(&task.1));
    let mut count = -1;
    for task in tasks.iter() {
        count += 1;
        match con.execute(update, params![count, task.0]) {
            Ok(_) => println!("updating task with id {}", task.0),
            Err(e) => panic!("could not update task with id: {}\nerror: {}", task.0, e),
        }
    }
    Ok(tasks.len())
}
