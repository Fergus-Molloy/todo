use crate::database::database;
use rusqlite::Result;

pub fn _new_list(name: String) -> i32 {
    match database::list_exists(Some(name.clone())) {
        Ok(id) => {
            println!("List already exists");
            id
        }
        Err(_) => {
            if database::user_agreement(format!(
                "List {} not recoginsed create new list? (y/n)",
                name
            )) {
                match create_list(name) {
                    Ok(v) => v as i32,
                    Err(e) => {
                        eprintln!("Could not create list: {}", e);
                        std::process::exit(1);
                    }
                }
            } else {
                eprintln!("User rejection, exiting");
                std::process::exit(1);
            }
        }
    }
}

pub fn create_list(name: String) -> Result<usize> {
    let con = database::connect().unwrap();
    let create = r"
    INSERT INTO lists (name, current, MaxNum) values(?, 0, 0);
    ";
    let mut stmt = con.prepare(create).unwrap();
    stmt.execute(params![name])
}

pub fn new_task(data: String, priority: i32, list: Option<String>) {
    let list_id = match database::list_exists(list) {
        Ok(val) => val,
        Err(e) => {
            eprintln!("Cannot update nums (list doesn't exist): {}", e);
            std::process::exit(1);
        }
    };
    let con = database::connect().unwrap();
    let p: i32 = priority.into();
    let d: String = data.into();
    let add_task = r"
    INSERT INTO tasks (num, complete, data, priority) values(
    (SELECT lists.MaxNum as max FROM lists where lists.id==:list), 0,:data,:p)";
    let add_task_to_list = r"
    INSERT INTO task_to_list (task, list) VALUES ((SELECT MAX(id) as task_id from tasks), :list)";
    let update_lists = r"
    UPDATE lists set MaxNum=(SELECT lists.MaxNum as max FROM lists where lists.id==:list)+1
    where lists.id==:list";

    con.execute_named(
        add_task,
        named_params! {":list": list_id, ":data": d, ":p": p},
    )
    .unwrap();
    con.execute_named(add_task_to_list, named_params! {":list": list_id})
        .unwrap();
    let stmt = con.execute_named(update_lists, named_params! {":list": list_id});
    match stmt {
        Ok(_) => println!(
            "Sucessfully added task to {}\n{}",
            database::get_list_name(list_id).expect("Something went worng getting list name"),
            d
        ),
        Err(e) => panic!("{}", e),
    }
}
