use clap::Parser;

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
    println!("{}", command)
}
