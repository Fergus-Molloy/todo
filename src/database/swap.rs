use crate::database::database;

pub fn swap(num_one: i32, num_two: i32, list: Option<String>) -> Result<()> {
    let list_id = match database::list_exists(list) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Cannot update nums (list doesn't exist): {}", e);
            std::process::exit(1);
        }
    };
    let con = database::connect().unwrap();

    let get_task_id = r"
    SELECT t.id from tasks as t
    INNER JOIN task_to_list ON t.id==task_to_list.task
    INNER JOIN lists ON task_to_list.list==lists.id
    WHERE lists.id==? AND t.num==?";
    let id1: i32 = con.query_row(
        get_task_id,
        params![list_id, num_one],
        |row| Ok(row.get(0)?),
    )?;
    let id2: i32 = con.query_row(
        get_task_id,
        params![list_id, num_two],
        |row| Ok(row.get(0)?),
    )?;

    let update = "UPDATE tasks SET num=? WHERE id==?";
    let _ = con.execute(update, params![num_two, id1]).unwrap();
    let _ = con.execute(update, params![num_one, id2]).unwrap();
    Ok(())
}
