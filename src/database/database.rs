use rusqlite::{Connection, Result, NO_PARAMS};
use std::fmt::Display;
use std::io;
use std::path::PathBuf;

/// Creates a new database if no existing database is found
pub fn db_get() -> PathBuf {
    let mut todo = dirs::home_dir().unwrap();
    todo.push(".todo.db");
    if !todo.exists() {
        println!("creating new db at {:?}", todo);
        println!("run `todo tasks` to see the tutorial",);
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
            "INSERT INTO lists (name, current, MaxNum) VALUES('Tutorial', 1, 0)",
            NO_PARAMS,
        )
        .unwrap();
        // add tutorial tasks
        super::add::new_task(String::from("Welcome to my todo app!\nYou can veiw your current tasks and lists using `todo tasks` and `todo lists`."), 2, Some(String::from("Tutorial"))).unwrap();
        super::add::new_task(String::from("Add tasks using the add command `todo add task`.\nTasks can have a low, medium or high priority which can be specified with the `-p` option."), 2, Some(String::from("Tutorial"))).unwrap();
        super::add::new_task(String::from("Complete tasks by using `todo complete <num>`.\nDon't worry they won't disapprear until you `todo clean` your list!"), 1, Some(String::from("Tutorial"))).unwrap();
        super::add::new_task(String::from("You can explore the rest of the interface by using the `-h` flag for help with commands."), 0, Some(String::from("Tutorial"))).unwrap();
        super::add::new_task(String::from("To start using, you can remove this list using `todo remove list Tutorial` and\nthen create a new list using `todo add list <list_name>`."), 0, Some(String::from("Tutorial"))).unwrap();
    }
    assert!(todo.exists() && todo.is_file());
    todo
}

/// Returns a connection to database
pub fn connect() -> Connection {
    let todo = db_get();
    Connection::open(todo).expect("Could not connect to database")
}

pub fn get_list_name(list_id: &i32) -> String {
    let con = connect();
    let sql = "SELECT name from lists where id==?";
    con.query_row(sql, params![list_id], |row| Ok(row.get(0)?))
        .expect("Cannot get list name")
}

pub fn list_exists(name: &Option<String>) -> Result<i32> {
    match name {
        Some(list_name) => {
            let con = connect();
            let get = "SELECT id FROM lists WHERE name==?";
            con.query_row(get, params![list_name], |row| Ok(row.get(0)?))
        }
        None => Ok(get_current_list_id()),
    }
}

/// Gets the name of the list given or the current active list if `list_name` is `None`
pub fn dynamic_list_name(list_name: &Option<String>) -> String {
    let list_id = match list_exists(&list_name) {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Could not get list name (list doesn't exist)");
            std::process::exit(1);
        }
    };
    get_list_name(&list_id)
}

pub fn task_exists(num: &i32, list_id: &i32) -> Result<i32> {
    let con = connect();
    let sql = r"SELECT tasks.id  FROM task_to_list
    INNER JOIN tasks ON tasks.id=task_to_list.task
    INNER JOIN lists ON lists.id=task_to_list.list
    WHERE lists.id==? ANd tasks.num=?";
    let mut stmt = con.prepare(sql).unwrap();
    stmt.query_row(params![list_id, num], |row| Ok(row.get(0)?))
}

pub fn get_current_list_id() -> i32 {
    let con = connect();
    let get = "SELECT id FROM lists WHERE current==1";
    let mut stmt = con.prepare(get).unwrap();
    let res = stmt.query_row(NO_PARAMS, |row| {
        let name: i32 = row.get(0)?;
        Ok(name)
    });
    match res {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Could not get current list's id (maybe no lists exist?)");
            if user_agreement("Would you like to create a new list? (y/n)") {
                println!("What would you like to call this list?");
                let mut name = String::new();
                io::stdin()
                    .read_line(&mut name)
                    .expect("Could not read input");
                let name = match name.strip_suffix("\n") {
                    Some(string) => string.to_owned(),
                    None => {
                        eprintln!("No name given aborting!");
                        std::process::exit(1);
                    }
                };
                super::add::new_list(name.clone());
                super::switch::switch_list(name).unwrap();
                get_current_list_id()
            } else {
                eprintln!("No current list created (user rejection)");
                std::process::exit(1);
            }
        }
    }
}

pub fn user_agreement<S: Display>(phrase: S) -> bool {
    println!("{}", phrase);
    let mut inp = String::new();
    io::stdin()
        .read_line(&mut inp)
        .expect("could not read input");
    let inp = inp.strip_suffix("\n").unwrap_or("n"); // default to reject if no input given
    inp == "y"
}
