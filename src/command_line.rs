use clap::Parser;
use std::fmt::Formatter;

const DIVIDER_STRING: &str = "-----------------";

#[derive(Parser)]
#[clap(author = "Mario Daniel Panuco")]
#[clap(version = "0.2.1")]
#[clap(about = "Command Line Countdown")]
#[clap(long_about = None)]
#[derive(Debug)]

pub struct Args {
    // #[clap(short = 't', long = "time", value_parser)]
    pub time: String,
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Arguments\n{}\n\
                Time: {}",
               DIVIDER_STRING, self.time)
    }
}