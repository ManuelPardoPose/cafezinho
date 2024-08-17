use clap::Parser;
use cafezinho::{commands::commands::{ls, reset}, file_io::file_utils};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Which command to execute
    #[arg(short, long, default_value_t = ("").to_string())]
    cmd: String,

    /// Debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    if file_utils::init_data_folder().is_err() {
        println!("ERROR WHILE CREATING DATA FOLDER");
        return;
    }

    match args.cmd.as_str() {
        "reset" => {
            reset(args.debug);
        },
        "lsdb" => {
            lsdb(args.debug);
        },
        _ => {},
    }
}
