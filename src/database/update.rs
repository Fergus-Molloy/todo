use crate::database::database;
use rusqlite::Result;

/// Re-oder the nums assigned to tasks such that there are no gaps
///
/// Takes a `Option<String>` for the name of the list to perform the update on.
/// `Some("name")` will perform update on the list called `name` whereas `None` will perform
/// update on the currently active list
///
/// #Panics
/// Panics when the list is not in the database
pub fn update_nums(list: Option<String>) -> Result<usize> {
    // check list exists and get list id
    let list_id = match database::list_exists(&list) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Cannot update nums (list doesn't exist): {}", e);
            std::process::exit(1);
        }
    };
    let con = database::connect();

    // select id, priority and num from all tasks in the list
    let get_tasks = r"
    SELECT t.id, t.priority, t.num FROM tasks AS t
    INNER JOIN task_to_list ON t.id==task_to_list.task
    INNER JOIN lists ON task_to_list.list==lists.id
    WHERE lists.id==?";
    let iter = con
        .query_map(get_tasks, params![list_id], |row| {
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
    // sort by priority
    tasks.sort_by(|task, other| other.1.cmp(&task.1));

    let update = "UPDATE tasks SET num=? WHERE id==?";
    let mut count = -1;
    for task in tasks.iter() {
        count += 1;
        match con.execute(update, params![count, task.0]) {
            Ok(_) => println!("updating task with id {} to num {}", task.0, count),
            Err(e) => panic!("could not update task with id: {}\nerror: {}", task.0, e),
        }
    }

    // update maxnum to reflect the tasks in the now condensed list
    let update_maxnum = "UPDATE lists SET MaxNum=:max WHERE id==:id";
    let res = con
        .execute_named(
            update_maxnum,
            named_params! {":id": list_id, ":max": tasks.len() as i32},
        )
        .unwrap();
    assert_eq!(res, 1);

    Ok(tasks.len())
}
