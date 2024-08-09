use clap::Parser;
use cafezinho::{database::db_utils, file_io::file_utils};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Which command to execute
    #[arg(short, long, default_value_t = ("").to_string())]
    command: String,
}

fn main() {
    let args = Args::parse();
    let command = args.command;
    println!("{}", command);

    if file_utils::init_data_folder().is_err() {
        println!("ERROR WHILE CREATING DATA FOLDER");
        return;
    }

    db_utils::setup_db();
    // file_utils::wipe_data_folder();
    db_utils::print_tables();
}
