#![warn(missing_docs)]
use crate::database::database;
use rusqlite::Result;

pub fn new_list(name: String) {
    match database::list_exists(&Some(name.clone())) {
        Ok(_) => {
            eprintln!("List `{}` already exists", name);
            std::process::exit(1);
        }
        Err(_) => match create_list(&name) {
            Ok(_) => println!("List `{}` sucessfully created", name),
            Err(e) => {
                eprintln!("Could not create list `{}`: {}", name, e);
                std::process::exit(1);
            }
        },
    }
}

/// Add a list with the given name to the database
///
/// Returns a result containing the number of rows affected
fn create_list(name: &String) -> Result<usize> {
    let con = database::connect();
    let create = "INSERT INTO lists (name, current, MaxNum) VALUES(?, 0, 0);";
    let mut stmt = con.prepare(create).unwrap();
    stmt.execute(params![name])
}

/// Add a new task to the database
///
/// Adds the task to the given list or the active list if no list is given
pub fn new_task(data: String, priority: i32, list: Option<String>) -> Result<usize> {
    // check list exists and get it's id
    let list_id = match database::list_exists(&list) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Cannot update nums (list doesn't exist): {}", e);
            std::process::exit(1);
        }
    };
    let con = database::connect();
    let p: i32 = priority;
    let d: String = data;

    let add_task = r"
    INSERT INTO tasks (num, complete, data, priority) VALUES(
    (SELECT lists.MaxNum AS max FROM lists where lists.id==?), 0,?,?)";
    con.execute(add_task, params![list_id, d, p])?;

    let add_task_to_list = r"
    INSERT INTO task_to_list (task, list)
    VALUES ((SELECT MAX(id) AS task_id from tasks), ?)";
    con.execute(add_task_to_list, params![list_id])?;

    let update_lists = r"
    UPDATE lists SET MaxNum=(
    SELECT lists.MaxNum AS max FROM lists WHERE lists.id==:list)+1
    WHERE lists.id==:list";
    con.execute_named(update_lists, named_params! {":list": list_id})
}
