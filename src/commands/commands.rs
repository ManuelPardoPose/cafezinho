use crate::database::db_utils;
use crate::file_io::file_utils;
use rusqlite::Error;

pub fn reset(debug_mode: bool) {
    println!("resetting the db...");
    file_utils::wipe_data_folder();
    let _ = file_utils::init_data_folder();
    match db_utils::setup_db() {
        Ok(()) => println!("SUCCESS"),
        Err(e) => {
            print_error(e, debug_mode);
        }
    }
}

pub fn lsdb(debug_mode: bool) {
    println!("listing the database");
    match db_utils::print_tables() {
        Err(e) => {
            print_error(e, debug_mode);
        }
        _ => {}
    }
}

pub fn drink(debug_mode: bool, coffee_type_str: String) {
    println!("drinking {}", coffee_type_str);
    println!("NOT IMPLEMENTED");
    match db_utils::add_entry(coffee_type_str) {
        Ok(()) => println!("SUCCESS"),
        Err(e) => {
            print_error(e, debug_mode);
        }
    }
}

pub fn print_error(e: Error, debug_mode: bool) {
    println!("ERROR");
    if debug_mode {
        println!("{:?}", e);
    }
}
