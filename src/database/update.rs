use crate::database::database;
use rusqlite::Result;

pub fn update_nums(list: Option<String>) -> Result<usize> {
    let list = list.unwrap_or(database::get_current_list_name());
    let sql = r"
    SELECT t.id, t.priority, t.num FROM tasks AS t
    INNER JOIN task_to_list ON t.id==task_to_list.task
    INNER JOIN lists ON task_to_list.list==lists.id
    WHERE lists.name==?";
    let update = "UPDATE tasks SET num=? WHERE id==?";
    let con = database::connect().unwrap();
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
