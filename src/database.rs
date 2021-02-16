use crate::task::{Priority, Task};
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};
use std::io;
use std::path::PathBuf;

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

fn connect() -> Result<Connection> {
    let todo = db_get();
    Connection::open(todo)
}

pub fn get_current_tasks() -> Vec<Task> {
    let (_, list) = get_current_list();
    get_tasks(list)
}

pub fn get_tasks(list: String) -> Vec<Task> {
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
/*
pub fn get_all_lists() -> Result<Vec<(i32, String)>> {
    let sql = "SELECT id, name FROM lists";
    let con = connect().unwrap();
    let mut stmt = con.prepare(sql).unwrap();
    let iter = stmt
        .query_map(NO_PARAMS, |row| {
            let id: i32 = row.get(0)?;
            let name: String = row.get(1)?;
            Ok((id, name))
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
}*/

pub fn new_task_current<S: Into<String>, P: Into<i32>>(data: S, priority: P) {
    let con = connect().unwrap();
    let get = r"
    SELECT name from lists where current==1";
    let name = con
        .query_row(get, NO_PARAMS, |row| {
            let name: String = row.get(0).unwrap();
            Ok(name)
        })
        .unwrap();
    new_task(data, priority, name);
}

pub fn remove_task(num: i32) {
    let con = connect().unwrap();
    let rm = r"
    DELETE FROM tasks WHERE tasks.num==?
    ";
    let mut stmt = con.prepare(rm).unwrap();
    stmt.execute(params![num]).unwrap();
    println!("Removed task {}", num);
}

pub fn remove_list(name: String) {
    let con = connect().unwrap();
    let sql = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.name==:name);
    DELETE from lists where lists.name==:name
    ";
    con.execute_named(sql, named_params! {":name": name})
        .unwrap();
    println!("Removed {}", name);
}

fn get_list(name: String) -> Result<i32> {
    let con = connect().unwrap();
    let get = r"
    SELECT id FROM lists
    WHERE name==?
    ";
    let result = con.query_row(get, params![name], |row| Ok(row.get(0)?))?;
    Ok(result)
}

fn create_list(name: String) -> i32 {
    let accept: [&str; 4] = ["y", "yes", "yeah", "yy"];
    println!("List {} not recoginsed create new list? (y/n)", name);
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("could not read input");

    if accept.iter().any(|&x| x == inp) {
        // create new list
        let con = connect().unwrap();
        let create = r"
        INSERT INTO lists (name, current) values(?, 0);
        ";
        let mut stmt = con.prepare(create).unwrap();
        stmt.execute(params![name]).unwrap();
        get_list(name).unwrap()
    } else {
        // user rejected cancelling opertaion
        panic!("cannot create list, reason: user cancelled");
    }
}

pub fn new_list<T: Into<String> + Clone>(name: T) -> i32 {
    let exists = get_list(name.clone().into());
    if exists.is_ok() {
        exists.unwrap()
    } else {
        create_list(name.into())
    }
}

pub fn new_task<S, P, T>(data: S, priority: P, list: T)
where
    S: Into<String>,
    P: Into<i32>,
    T: Into<String> + Clone,
{
    let list_id = new_list(list.clone());
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
        Ok(_) => println!("Sucessfully added task to {}\n{}", list.into(), d),
        Err(e) => panic!("{}", e),
    }
}

pub fn complete(num: i32) {
    // find and update task to be complete
    let con = connect().unwrap();
    let update = r"
    UPDATE tasks SET complete==1 WHERE num==?
    ";
    match con.execute(update, params![num]) {
        Ok(_) => println!("Completed {:03}", num),
        Err(e) => panic!("could not update {}:\n{}", num, e),
    }
}

pub fn update_current_nums() -> Result<usize> {
    let (_, list) = get_current_list();
    update_nums(list)
}

pub fn update_nums(list: String) -> Result<usize> {
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

pub fn update_desc(num: i32, data: String) {
    let (_, list) = get_current_list();
    update_desc_list(num, data, list)
}
pub fn update_desc_list(num: i32, data: String, list: String) {
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
    .unwrap();
}

pub fn clean_current() -> Result<usize> {
    let (_, list) = get_current_list();
    clean(list)
}

pub fn clean(list: String) -> Result<usize> {
    // find completed tasks and remove them
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

pub fn switch_list(name: String) {
    match get_list(name) {
        Ok(list) => {
            let con = connect().unwrap();
            let update = r"UPDATE lists SET current=0 where current=1";
            let _ = con.execute(update, NO_PARAMS).unwrap();
            let set = r"UPDATE lists SET current=1 WHERE name==?";
            con.execute(set, params![list]).unwrap();
            println!("Set current list to {}", list);
        }
        Err(_) => eprintln!("list does not exist"),
    }
}

pub fn get_current_list() -> (i32, String) {
    let con = connect().unwrap();
    let get = "SELECT id, name FROM lists WHERE current==1";
    let mut stmt = con.prepare(get).unwrap();
    let res = stmt.query_row(NO_PARAMS, |row| {
        let id: i32 = row.get(0)?;
        let name: String = row.get(1)?;
        Ok((id, name))
    });
    res.unwrap()
}

pub fn swap_current(num_one: i32, num_two: i32) {
    let (_, list) = get_current_list();
    swap(num_one, num_two, list)
}
pub fn swap(num_one: i32, num_two: i32, list: String) {
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
