use super::database;
use rusqlite::{Result, NO_PARAMS};

pub fn get_all_list_names() -> Result<Vec<(String, bool)>> {
    let sql = "SELECT name, current FROM lists";
    let con = database::connect();
    let mut stmt = con.prepare(sql).unwrap();
    let iter = stmt
        .query_map(NO_PARAMS, |row| {
            let name: String = row.get(0)?;
            let current: bool = row.get(1)?;
            Ok((name, current))
        })
        .unwrap();
    let mut lists = Vec::new();
    for item in iter {
        match item {
            Ok(val) => lists.push(val),
            Err(e) => panic!("could not get all lists {}", e),
        }
    }
    Ok(lists)
}
