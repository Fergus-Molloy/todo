use crate::database::database;
use rusqlite::Result;

pub fn complete(num: i32, list: Option<String>) -> Result<usize> {
    let con = database::connect().unwrap();
    // get id of list to be queried
    let list = list.unwrap_or(database::get_current_list_name());
    let list_id = crate::database::add::new_list(&list);
    // get tasks that belong to the desired list with the number given that are completed
    let already_complete_query = r"
    SELECT t.complete, t.num FROM tasks AS t
    INNER JOIN task_to_list ON t.id==task_to_list.task
    INNER JOIN lists ON task_to_list.list==lists.id
    WHERE lists.id==? AND t.num==? AND complete==1";
    // execute statement
    let mut stmt = con.prepare(already_complete_query).unwrap();
    let res = stmt
        .query_map(params![list_id, num], |row| {
            let id: i32 = row.get(0)?;
            let complete: bool = row.get(1)?;
            Ok((id, complete))
        })
        .unwrap();
    let already_complete = res.count() == 1; // count rows returned (1 means already completed)

    // create query to update completeness
    // set to 0 if already complete otherwise set to 1
    let update = format!(
        "UPDATE tasks SET complete=={} WHERE num==?",
        if already_complete { 0 } else { 1 }
    );
    con.execute(&update, params![num])
}