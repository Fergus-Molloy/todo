use crate::database::database;

// Update
pub fn swap(num_one: i32, num_two: i32, list: Option<String>) {
    let list = list.unwrap_or(database::get_current_list_name());
    let sql = r"
    SELECT t.id from tasks as t
    inner join task_to_list on t.id==task_to_list.task
    inner join lists on task_to_list.list=lists.id
    where lists.name==? and t.num=?";
    let con = database::connect().unwrap();
    let id1: i32 = con
        .query_row(sql, params![list, num_one], |row| Ok(row.get(0)?))
        .unwrap();
    let id2: i32 = con
        .query_row(sql, params![list, num_two], |row| Ok(row.get(0)?))
        .unwrap();
    let update = "UPDATE tasks SET num=? WHERE id==?";
    let _ = con.execute(update, params![num_two, id1]).unwrap();
    let _ = con.execute(update, params![num_one, id2]).unwrap();
}
