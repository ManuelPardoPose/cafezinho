use std::{fs, io};

pub const DATA_FOLDER_PATH: &str = "./.data/";

pub fn data_folder_exists() -> bool {
    fs::metadata(DATA_FOLDER_PATH).is_ok()
}

pub fn init_data_folder() -> io::Result<()> {
    if data_folder_exists() {
        return Ok(());
    }
    return fs::create_dir(DATA_FOLDER_PATH);
}

pub fn wipe_data_folder() {
    if !data_folder_exists() {
        return;
    }
    let _ = fs::remove_dir_all(DATA_FOLDER_PATH);
}
