use crate::database::db_utils;
use crate::file_io::file_utils;

pub fn reset(debug_mode: bool) {
    println!("resetting the db...");
    file_utils::wipe_data_folder();
    let _ = file_utils::init_data_folder();
    match db_utils::setup_db() {
        Ok(()) => println!("SUCCESS"),
        Err(e) => {
            println!("ERROR");
            if debug_mode {
                println!("{:?}", e);
            }
        }
    }
}

pub fn lsdb(debug_mode: bool) {
    println!("listing the database");
    match db_utils::print_tables() {
        Err(e) => {
            println!("ERROR");
            if debug_mode {
                println!("{:?}", e);
            }
        }
        _ => {}
    }
}
