use crate::database::database;
use rusqlite::Result;

pub fn update_nums(list: Option<String>) -> Result<usize> {
    let list_id = match database::list_exists(list) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Cannot update nums (list doesn't exist): {}", e);
            std::process::exit(1);
        }
    };
    let sql = r"
    SELECT t.id, t.priority, t.num FROM tasks AS t
    INNER JOIN task_to_list ON t.id==task_to_list.task
    INNER JOIN lists ON task_to_list.list==lists.id
    WHERE lists.id==?";
    let update = "UPDATE tasks SET num=? WHERE id==?";
    let con = database::connect().unwrap();
    let mut stmt = con.prepare(sql).unwrap();
    let iter = stmt
        .query_map(params![list_id], |row| {
            let id: i32 = row.get(0)?;
            let p: i32 = row.get(1)?;
            let num: i32 = row.get(2)?;
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
            Ok(_) => println!("updating task with id {} to num {}", task.0, count),
            Err(e) => panic!("could not update task with id: {}\nerror: {}", task.0, e),
        }
    }
    Ok(tasks.len())
}
