# Rusty Timer 
A command line countdown utility 
## Warning 
This project is still very much a work in progress. I plan on finishing most of it by July 2nd, 2022. 
## Goals 
* Accuracy - Currently accurate to the micro-second (thousandth of a second) 
* Readability 
* Customization through imported font files 
## Usage 
Working on publishing to crate.io
For now clone repo, install rustup from Rust's website, and run "cargo build --release"  
The binary can be found in rusty_timer/target/release
```
rustytimer [OPTIONS] [TIME]
Example: 
$ rustytimer 3m
    This runs the timer from 3 minutes to 0
```

## TODOS
* Font customization and output 
* Remove second of wait during first iteration of countdown loop 
