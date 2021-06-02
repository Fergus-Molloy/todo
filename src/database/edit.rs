use crate::database::database;
use rusqlite::Result;

pub fn update_desc(num: i32, data: String, list: Option<String>) -> Result<usize> {
    let list_id = match database::list_exists(list) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Cannot update nums (list doesn't exist): {}", e);
            std::process::exit(1);
        }
    };
    let sql = r"
    UPDATE tasks AS t SET data=? WHERE t.id IN (SELECT tt.id FROM tasks AS tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.id==? AND tt.num==?);";
    let con = database::connect().unwrap();
    con.execute(sql, params![data, list_id, num])
}
