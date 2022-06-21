mod command_line;

use clap::Parser;

fn main() {
    let args: command_line::Args = command_line::Args::parse();

    println!("{}", args.time);

}


fn turn_time_to_seconds(time: String) -> i32 {
    unimplemented!()
}


