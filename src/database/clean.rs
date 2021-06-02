use crate::database::database;
use rusqlite::Result;

// Delete
pub fn clean(list_name: Option<String>) -> Result<usize> {
    // find completed tasks and remove them
    let list_id = match database::list_exists(list_name) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error list cannot be cleaned (does not exist): {}", e);
            std::process::exit(1);
        }
    };

    println!("Cleaning {}", list_id);
    let con = database::connect().unwrap();
    let remove = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.id==? AND tt.complete==1);
    ";
    let mut stmt = con.prepare(remove).unwrap();
    stmt.execute(params![format!("{}", list_id)])
}
