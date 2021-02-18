use crate::task::{Priority, Task};
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::fmt::Display;
use std::io;
use std::path::PathBuf;

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
fn connect() -> Result<Connection> {
    let todo = db_get();
    Connection::open(todo)
}

// Delete
pub fn clean(list: Option<String>) -> Result<usize> {
    // find completed tasks and remove them
    let list = list.unwrap_or(get_current_list_name());
    println!("Cleaning {}", list);
    let con = connect().unwrap();
    let remove = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.name==? AND tt.complete==1);
    ";
    let mut stmt = con.prepare(remove).unwrap();
    stmt.execute(params![format!("{}", list)])
}

// needs redoing now multiple tasks can have the same num
// Delete
pub fn remove_task(num: i32, list: Option<String>) {
    let list = list.unwrap_or(get_current_list_name());
    let con = connect().unwrap();
    let sql = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.name==? AND tt.num==?);
    ";
    let mut stmt = con.prepare(sql).unwrap();
    stmt.execute(params![list, num]).unwrap();
    let sql = r"
    UPDATE lists SET MaxNum=(
    SELECT MaxNum FROM lists WHERE name=:name)-1 WHERE name=:name
    ";
    let mut stmt = con.prepare(sql).unwrap();
    stmt.execute_named(named_params! {":name": list}).unwrap();
    println!("Removed task {}", num);
}

// Delete
pub fn remove_list(name: String) {
    let con = connect().unwrap();
    let sql = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.name==?)";
    con.execute(sql, params![name]).unwrap();
    let sql = r"
    DELETE from lists where lists.name==?
    ";
    con.execute(sql, params![name]).unwrap();
    println!("Removed {}", name);
}

// fetch
pub fn get_tasks(list: Option<String>) -> Vec<Task> {
    let list = list.unwrap_or(get_current_list_name());
    let con = connect().unwrap();
    let get = r"
    SELECT tasks.*, lists.name FROM task_to_list
    INNER JOIN tasks ON tasks.id=task_to_list.task
    INNER JOIN lists ON lists.id=task_to_list.list
    WHERE lists.name==?
    ";
    let mut stmt = con.prepare(get).unwrap();
    let stmt_iter = stmt
        .query_map(params![list], |row| {
            Ok(Task {
                id: row.get(0)?,
                num: row.get(1)?,
                complete: {
                    let res: i32 = row.get(2)?;
                    res == 1
                },
                priority: {
                    let res: i32 = row.get(3)?;
                    Priority::new(res)
                },
                data: row.get(4)?,
                list: row.get(5)?,
            })
        })
        .unwrap();

    let mut tasks: Vec<Task> = Vec::new();
    for task in stmt_iter {
        tasks.push(task.unwrap());
    }
    tasks
}

// fetch
pub fn get_all_list_names() -> Result<Vec<String>> {
    let sql = "SELECT name FROM lists";
    let con = connect().unwrap();
    let mut stmt = con.prepare(sql).unwrap();
    let iter = stmt
        .query_map(NO_PARAMS, |row| {
            let name: String = row.get(0)?;
            Ok(name)
        })
        .unwrap();
    let mut lists = Vec::new();
    for item in iter {
        match item {
            Ok(val) => lists.push(val),
            Err(e) => panic!("could not get all lists {}", e),
        }
    }
    Ok(lists)
}

// fetch
fn get_list_id(name: &String) -> Result<i32> {
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

// insert
pub fn new_task(data: String, priority: i32, list: Option<String>) {
    let list = list.unwrap_or(get_current_list_name());
    let list_id = new_list(&list);
    let con = connect().unwrap();
    let p: i32 = priority.into();
    let d: String = data.into();
    let add_task = r"
    INSERT INTO tasks (num, complete, data, priority) values(
    (SELECT lists.MaxNum as max FROM lists where lists.id==:list), 0,:data,:p)";
    let add_task_to_list = r"
    INSERT INTO task_to_list (task, list) VALUES ((SELECT MAX(id) as task_id from tasks), :list)";
    let update_lists = r"
    UPDATE lists set MaxNum=(SELECT lists.MaxNum as max FROM lists where lists.id==:list)+1
    where lists.id==:list";

    con.execute_named(
        add_task,
        named_params! {":list": list_id, ":data": d, ":p": p},
    )
    .unwrap();
    con.execute_named(add_task_to_list, named_params! {":list": list_id})
        .unwrap();
    let stmt = con.execute_named(update_lists, named_params! {":list": list_id});
    match stmt {
        Ok(_) => println!("Sucessfully added task to {}\n{}", list, d),
        Err(e) => panic!("{}", e),
    }
}

// insert
pub fn create_list(name: &String) -> Result<usize> {
    let con = connect().unwrap();
    let create = r"
    INSERT INTO lists (name, current, MaxNum) values(?, 0, 0);
    ";
    let mut stmt = con.prepare(create).unwrap();
    stmt.execute(params![name])
}

fn user_agreement<S: Display>(phrase: S) -> bool {
    let accept_phrases: [&str; 4] = ["y", "yes", "yeah", "yy"];
    println!("{}", phrase);
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("could not read input");
    accept_phrases.iter().any(|&x| x == inp)
}

// insert
fn new_list(name: &String) -> i32 {
    match get_list_id(name) {
        Ok(id) => id, //list already exists
        Err(_) => {
            if user_agreement(format!(
                "List {} not recoginsed create new list? (y/n)",
                name
            )) {
                create_list(name.into())
            } else {
                eprintln!("User rejection, exiting");
                std::process::exit(1);
            }
        }
    }
}

// needs updating so a list may be specified
// Update
pub fn complete(num: i32) -> Result<usize> {
    // find and update task to be complete
    let con = connect().unwrap();
    let update = r"
    UPDATE tasks SET complete==1 WHERE num==?
    ";
    con.execute(update, params![num])
}

// update
pub fn update_nums(list: Option<String>) -> Result<usize> {
    let list = list.unwrap_or(get_current_list_name());
    let sql = r"
    SELECT t.id, t.priority, t.num FROM tasks AS t
    INNER JOIN task_to_list ON t.id==task_to_list.task
    INNER JOIN lists ON task_to_list.list=lists.id
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

// Update
pub fn update_desc(num: i32, data: String, list: Option<String>) -> Result<usize> {
    let list = list.unwrap_or(get_current_list_name());
    let sql = r"
    UPDATE tasks AS t SET data=:data WHERE t.id IN (SELECT tt.id FROM tasks AS tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.name==:list AND tt.num==:num);";
    let con = connect().unwrap();
    con.execute_named(
        sql,
        named_params! {":list": list, ":data": data, ":num": num},
    )
}

// Update
pub fn switch_list(name: &String) -> Result<usize> {
    match get_list_id(name) {
        Ok(list) => {
            let con = connect().unwrap();
            let update = r"UPDATE lists SET current=0 where current=1";
            let _ = con.execute(update, NO_PARAMS).unwrap();
            let set = r"UPDATE lists SET current=1 WHERE name==?";
            con.execute(set, params![list])
        }
        Err(_) => {
            eprintln!("list does not exist");
            std::process::exit(1);
        }
    }
}

// Update
pub fn swap(num_one: i32, num_two: i32, list: Option<String>) {
    let list = list.unwrap_or(get_current_list_name());
    let sql = r"
    SELECT t.id from tasks as t
    inner join task_to_list on t.id==task_to_list.task
    inner join lists on task_to_list.list=lists.id
    where lists.name==? and t.num=?";
    let con = connect().unwrap();
    let id1: i32 = con
        .query_row(sql, params![list, num_one], |row| Ok(row.get(0)?))
        .unwrap();
    let id2: i32 = con
        .query_row(sql, params![list, num_two], |row| Ok(row.get(0)?))
        .unwrap();
    let update = "UPDATE tasks SET num=? WHERE id==?";
    let _ = con.execute(update, params![num_two, id1]).unwrap();
    let _ = con.execute(update, params![num_one, id2]).unwrap();
}
