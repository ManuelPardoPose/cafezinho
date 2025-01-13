use crate::database::db_utils;
use crate::file_io::file_utils;
use rusqlite::Error;

pub fn reset(debug_mode: bool) {
    file_utils::wipe_data_folder();
    let _ = file_utils::init_data_folder();
    match db_utils::setup_db() {
        Ok(()) => println!("SUCCESS"),
        Err(e) => {
            print_db_error(e, debug_mode);
        }
    }
}

pub fn lsdb(debug_mode: bool) {
    match db_utils::print_tables() {
        Err(e) => {
            print_db_error(e, debug_mode);
        }
        _ => {}
    }
}

pub fn drink(debug_mode: bool, coffee_type_str: String) {
    match db_utils::add_entry(coffee_type_str) {
        Ok(()) => println!("SUCCESS"),
        Err(e) => {
            print_error(e, debug_mode);
        }
    }
}

pub fn stats(debug_mode: bool) {
    match db_utils::print_stats() {
        Err(e) => {
            print_db_error(e, debug_mode);
        }
        _ => {}
    }
}

pub fn print_db_error(e: Error, debug_mode: bool) {
    println!("ERROR");
    if debug_mode {
        println!("{:?}", e);
    }
}

pub fn print_error(e: &str, debug_mode: bool) {
    println!("ERROR");
    if debug_mode {
        println!("{:?}", e);
    }
}
