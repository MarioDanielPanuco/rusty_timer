use clap::Parser;
use std::fmt::Formatter;

const DIVIDER_STRING: &str = "-----------------";

#[derive(Parser)]
#[clap(author = "Daniel_Panuco")]
#[clap(version = "0.01.12")]
#[clap(about = "Command line tool for encrypting files")]
#[clap(long_about = None)]
#[derive(Debug)]

pub struct Args {
    #[clap(short = 'f', long = "file", value_parser)]
    pub file: std::path::PathBuf,

    #[clap(short = 'k', long = "key", value_parser)]
    pub key: std::path::PathBuf,

    #[clap(short = 'r', long = "random", value_parser)]
    pub rand: Option<i32>,
}

impl std::fmt::Display for Args {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,
               "Arguments\n{}\n\
            Plain Text Path: {:?}\n\
            Key Path: {:?}\n\
            Random: {:?}",
               DIVIDER_STRING, self.file, self.key, self.rand)
    }
}