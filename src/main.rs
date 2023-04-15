mod clip_db;
use clap::Parser;
use clip_db::*;
use diesel::prelude::*;
use diesel_migrations::MigrationHarness;
use diesel_migrations::{embed_migrations, EmbeddedMigrations};
use std::io::{self, Read};
use std::string::FromUtf8Error;
mod iced_gui;
use log::warn;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    store: bool,
    #[arg(short, long)]
    list: bool,
    #[arg(short, long)]
    gui: bool,
    #[arg(short, long)]
    clear: bool,
}

pub struct DieselDeps{
    pub conn: SqliteConnection,
    pub migrations:*mut EmbeddedMigrations,
}

    pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");
fn main() {
    // pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations/");
    let args = Args::parse();
    let mut conn = clip_db::establish_connection();
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run pending Migrations");
    if args.store {
        // save_copied_val(& conn)
        match get_stdin(){
            Err(err)=>warn!("Error converting copied value to utf8\n{:?}",err),
            Ok(clip_entry)=>save_copied_val(&mut conn,MIGRATIONS,clip_entry.as_str()),
        }
    } else if args.list {
        let clip_hist_iter = retrieve_clipboard_history(&mut conn);
        print_clipboard(clip_hist_iter)
    } else if args.gui {
        // let clip_hist_iter = retrieve_clipboard_history(&mut conn);
        iced_gui::show(conn).unwrap();
    }
    else if args.clear {
        revert_migrations(&mut conn, MIGRATIONS)
    }
    else {
        println!("Invalid Arguments Supplied")
    }
}

fn print_clipboard(clip_hist:Vec<ClipboardEntry>){
    println!("Total {} entries", clip_hist.len());
    for (index,retrieved_entry) in clip_hist.iter().enumerate() {
        // println!("{}", retrieved_entry.id);
        // println!("----------\n");
        println!("{}| {}",index, retrieved_entry.clip_text);
        //println!("----------");
    }
}
fn get_stdin()->Result<String, FromUtf8Error>{
    let mut bytes = Vec::new();
    io::stdin()
        .read_to_end(&mut bytes)
        .expect("No arguments supplied");
    let clipboard_entry = String::from_utf8(bytes)?;
    Ok(clipboard_entry)
}
fn save_copied_val(conn: &mut SqliteConnection,migrations:EmbeddedMigrations , clipboard_entry:&str) {

    remove_duplicates(conn, clipboard_entry);
    match write_to_db(conn, clipboard_entry) {
        Ok(result) => {
            println!("{:?}", result)
        }
        Err(error_val) => {
            println!("{:?}", error_val);

            if let Err(_error)=write_to_db(conn, &clipboard_entry){
                conn.run_pending_migrations(migrations).unwrap();
                write_to_db(conn, &clipboard_entry).expect("Error Occured even after creating a table with default values");
            };
        }
    }
}


fn revert_migrations(conn:&mut SqliteConnection, migrations:EmbeddedMigrations){
    conn.revert_last_migration(migrations).expect("Error reverting changes to the database");
}
