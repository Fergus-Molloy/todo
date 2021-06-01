use crate::database::database;
use rusqlite::Result;

pub fn update_desc(num: i32, data: String, list: Option<String>) -> Result<usize> {
    let list = list.unwrap_or(database::get_current_list_name());
    let sql = r"
    UPDATE tasks AS t SET data=:data WHERE t.id IN (SELECT tt.id FROM tasks AS tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.name==:list AND tt.num==:num);";
    let con = database::connect().unwrap();
    con.execute_named(
        sql,
        named_params! {":list": list, ":data": data, ":num": num},
    )
}
