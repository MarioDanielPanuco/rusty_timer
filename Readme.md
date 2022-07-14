# Rusty Timer 
A command line countdown utility 
## Warning 
This project is still a work in progress, but the basic functionality works. 

## Current Working Features 
* Functions as a stopwatch (currently accurate to the micro-second (thousandth of a second)) 
* Clears terminal before printing the updated time 
* Parses command line properly
* Linux Support

## Project Goals 
* Accuracy 
* Readability 
* Customization through imported font files

## Usage 
Working on publishing to crate.io
For now clone repo, install rustup from Rust's website, and run "cargo build --release"  
The binary can be found in rusty_timer/target/release
```
rustytimer [TIME] [OPTIONS]
Example: 
$ rustytimer 3m -b black -c red
    This runs the timer from 3 minutes to 0  
    with black background and red text
```

## Missing Features
* Font customization and output 
* Mac and Windows release binary support
