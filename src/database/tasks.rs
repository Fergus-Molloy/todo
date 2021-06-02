use crate::database::database;
use crate::task::{Priority, Task};

pub fn get_tasks(list: Option<String>) -> Vec<Task> {
    let list_id = match database::list_exists(list) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Cannot list tasks (list doesn't exist): {}", e);
            std::process::exit(1);
        }
    };
    let con = database::connect().unwrap();
    let get = r"
    SELECT tasks.*, lists.name FROM task_to_list
    INNER JOIN tasks ON tasks.id=task_to_list.task
    INNER JOIN lists ON lists.id=task_to_list.list
    WHERE lists.id==?
    ";
    let mut stmt = con.prepare(get).unwrap();
    let stmt_iter = stmt
        .query_map(params![list_id], |row| {
            let task = Task {
                id: row.get(0)?,
                num: row.get(1)?,
                complete: row.get(2)?,
                priority: {
                    let res: i32 = row.get(3)?;
                    Priority::new(res)
                },
                data: row.get(4)?,
                list: row.get(5)?,
            };
            Ok(task)
        })
        .unwrap();

    let mut tasks: Vec<Task> = Vec::new();
    for task in stmt_iter {
        tasks.push(task.unwrap());
    }
    tasks
}
