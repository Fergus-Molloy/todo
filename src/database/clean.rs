use crate::database::database;
use rusqlite::Result;

// Delete
pub fn clean(list: Option<String>) -> Result<usize> {
    // find completed tasks and remove them
    let list = list.unwrap_or(database::get_current_list_name());
    println!("Cleaning {}", list);
    let con = database::connect().unwrap();
    let remove = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.name==? AND tt.complete==1);
    ";
    let mut stmt = con.prepare(remove).unwrap();
    stmt.execute(params![format!("{}", list)])
}
