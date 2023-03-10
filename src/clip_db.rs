mod schema;
use diesel::{insert_into, prelude::*};
use clipboard_entries::dsl::*;
use schema::clipboard_entries;
#[derive(Queryable, PartialEq, Debug)]

pub struct ClipboardEntry {
    pub id: i32,
    pub clip_text: String,
}



pub fn write_to_db(connection: &mut SqliteConnection, clip_entry: &str) -> QueryResult<usize> {
    insert_into(clipboard_entries)
        .values(clip_text.eq(clip_entry))
        .execute(connection)
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = "./dbase.sqlite";
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn retrieve_clipboard_history(connection: &mut SqliteConnection) -> Vec<ClipboardEntry> {

    let results = clipboard_entries
        // .filter(published.eq(true))
        .limit(200)
        .load::<ClipboardEntry>(connection)
        .expect("Error loading clipboard entries");

    return results;
}
pub fn remove_duplicates(connection: &mut SqliteConnection, query: &String) {
    if let Err(_deleted) =
        diesel::delete(clipboard_entries.filter(clip_text.like(query))).execute(connection)
    {
        println!("No duplicates found")
    };
}
