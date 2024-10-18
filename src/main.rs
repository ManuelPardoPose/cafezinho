use cafezinho::{
    commands::commands::{drink, lsdb, reset},
    file_io::file_utils,
};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Reset Database
    #[arg(short, long, default_value_t = false)]
    reset: bool,

    /// List Database
    #[arg(short, long, default_value_t = false)]
    lsdb: bool,

    /// Add a consumption entry
    #[arg(short, long, default_value_t = String::from(""))]
    drink: String,

    /// Debug mode
    #[arg(long, default_value_t = false)]
    debug: bool,
}

fn main() {
    let args = Args::parse();

    if file_utils::init_data_folder().is_err() {
        println!("ERROR WHILE CREATING DATA FOLDER");
        return;
    }

    if args.reset {
        reset(args.debug);
    }

    if args.drink != "" {
        drink(args.debug, args.drink);
    }

    if args.lsdb {
        lsdb(args.debug);
    }
}
