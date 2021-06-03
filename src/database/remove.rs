use crate::database::database;

pub fn remove_task(num: i32, list: Option<String>) {
    // get id of list
    let list_id = match database::list_exists(list) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Cannot update nums (list doesn't exist): {}", e);
            std::process::exit(1);
        }
    };
    // get id of task
    let task_id = match database::task_exists(num, list_id) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Cannot remove task (task doesn't exist): {}", e);
            std::process::exit(1);
        }
    };
    let con = database::connect().unwrap(); // connect to db

    // delete task from table `tasks` where num and list id match (should be 1 row)
    let del_task = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id==task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.id==? AND tt.id==?)";
    let mut stmt = con.prepare(del_task).unwrap();
    let res = stmt.execute(params![list_id, task_id]).unwrap();
    assert_eq!(res, 1);

    // delete task from table `task_to_list` where the id of the task matches (should be 1 row)
    let del_task_to_list = r" DELETE FROM task_to_list WHERE task=?";
    let mut stmt = con.prepare(del_task_to_list).unwrap();
    let res = stmt.execute(params![task_id]).unwrap();
    assert_eq!(res, 1);

    // Update the maxnum in `lists` to reflect the new number of tasks (should change 1 row)
    let sql = r"
    UPDATE lists SET MaxNum=(
    SELECT MaxNum FROM lists WHERE id=:id)-1 WHERE id=:id";
    let mut stmt = con.prepare(sql).unwrap();
    let res = stmt.execute_named(named_params! {":id": list_id}).unwrap();
    assert_eq!(res, 1);

    println!("Removed task {}", num);
}

pub fn remove_list(list_name: String) {
    let list_id = match database::list_exists(Some(list_name.clone())) {
        Ok(id) => id,
        Err(_) => {
            eprintln!("Cannot remove list (doesn't exist)");
            std::process::exit(1);
        }
    };
    let con = database::connect().unwrap();

    if database::user_agreement(format!(
        r"Are you sure you want to delete {}?
        All associated tasks will also be deleted (y/n)",
        list_name
    )) {
        // delete all tasks associated with list
        let del_tasks = r"
    DELETE from tasks as t where t.id IN (select tt.id from tasks as tt
    INNER JOIN task_to_list ON tt.id== task_to_list.task
    INNER JOIN lists ON lists.id==task_to_list.list
    WHERE lists.id==?)";
        let count = con.execute(del_tasks, params![list_id]).unwrap();
        println!(
            "Removed {} task{}",
            count,
            if count == 1 { "" } else { "s" }
        );

        // delete the list
        let del_lists = "DELETE from lists where lists.id==?";
        con.execute(del_lists, params![list_id]).unwrap();

        // delete all relations between the list and tasks
        let del_task_to_list = "DELETE FROM task_to_list WHERE list=?";
        con.execute(del_task_to_list, params![list_id]).unwrap();

        println!("Removed {}", list_name);
    } else {
        println!("Did not delete list (user aborted)");
    }
}
