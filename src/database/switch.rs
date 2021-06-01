use crate::database::database;
use rusqlite::Result;
use rusqlite::NO_PARAMS;

pub fn switch_list(name: &String) -> Result<usize> {
    match database::get_list_id(name) {
        Ok(list) => {
            let con = database::connect().unwrap();
            let update = r"UPDATE lists SET current=0 where current=1";
            let _ = con.execute(update, NO_PARAMS).unwrap();
            let set = r"UPDATE lists SET current=1 WHERE name==?";
            con.execute(set, params![list])
        }
        Err(_) => {
            eprintln!("list does not exist");
            std::process::exit(1);
        }
    }
}
