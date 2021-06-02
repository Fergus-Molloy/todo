use crate::database::database;

//TODO: remove task_to_list item too

// needs redoing now multiple tasks can have the same num
// Delete
pub fn remove_task(num: i32, list: Option<String>) {
    let list = list.unwrap_or(database::get_current_list_name());
    let con = database::connect().unwrap();
    let sql = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.name==? AND tt.num==?);
    ";
    let mut stmt = con.prepare(sql).unwrap();
    stmt.execute(params![list, num]).unwrap();
    let sql = r"
    UPDATE lists SET MaxNum=(
    SELECT MaxNum FROM lists WHERE name=:name)-1 WHERE name=:name
    ";
    let mut stmt = con.prepare(sql).unwrap();
    stmt.execute_named(named_params! {":name": list}).unwrap();
    println!("Removed task {}", num);
}

// Delete
pub fn remove_list(name: String) {
    let con = database::connect().unwrap();
    let sql = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.name==?)";
    con.execute(sql, params![name]).unwrap();
    let sql = r"
    DELETE from lists where lists.name==?
    ";
    con.execute(sql, params![name]).unwrap();
    println!("Removed {}", name);
}
