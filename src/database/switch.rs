use crate::database::database;
use rusqlite::Result;
use rusqlite::NO_PARAMS;

pub fn switch_list(name: String) -> Result<usize> {
    match database::list_exists(Some(name)) {
        Ok(list) => {
            let con = database::connect().unwrap();
            let update = "UPDATE lists SET current=0 where current=1";
            let _ = con.execute(update, NO_PARAMS).unwrap();
            let set = "UPDATE lists SET current=1 WHERE id=?";
            con.execute(set, params![list])
        }
        Err(_) => {
            eprintln!("list does not exist");
            std::process::exit(1);
        }
    }
}
