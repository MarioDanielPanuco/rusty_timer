mod command_line;

use clap::Parser;
use regex::Regex;
use std::time;

const DATE_PATTERN: &str = r"((?P<years>\d+)y)?\
(?P<days>\d+)d)?\
((?P<hours>\d+)h)?\
((?P<minutes>\d+)m)?\
((?P<seconds>\d+)s)?";
use std::thread;

fn main() {
    let args: command_line::Args = command_line::Args::parse();
    let start = time::Instant::now();
    println!("{}", DATE_PATTERN);

    let duration = parse_input(args.time);

    println!("{:?}", duration);
    println!("{:?}", start);

    loop {
        let elapsed = start.elapsed();
        println!("{}", elapsed.as_secs());

        if duration < elapsed {
            break;
        }

        let remain = duration.as_secs() - elapsed.as_secs();
        std::thread::sleep(time::Duration::from_millis(1000));
        println!("Time Left: {}", remain);
    }
}

fn parse_input(duration: String) -> time::Duration {
    let re = Regex::new(r"((?P<years>\d+)y)?((?P<days>\d+)d)?
((?P<hours>\d+)h)?
((?P<minutes>\d+)m)?
((?P<seconds>\d+)s)?")
        .unwrap();
    //let re = Regex::new(DATE_PATTERN).unwrap();
    let caps = re.captures(&*duration).unwrap();

    let y: u64 = caps.name("years").map_or(0, |m| m.as_str().parse().unwrap());
    let d: u64 = caps.name("days").map_or(0, |m| m.as_str().parse().unwrap());
    let h: u64 = caps.name("hours").map_or(0, |m| m.as_str().parse().unwrap());
    let m: u64 = caps.name("minutes").map_or(0, |m| m.as_str().parse().unwrap());
    let s: u64 = caps.name("seconds").map_or(0, |m| m.as_str().parse().unwrap());
    time::Duration::new(36000 * h + 60 * m + s, 0)
}

fn temp() {
    let args: command_line::Args = command_line::Args::parse();
    let re = Regex::new(DATE_PATTERN).unwrap();

    println!("Time: {}", args.time);

    let caps = re.captures(&*args.time).unwrap();
    println!("Length of capture: {}", caps.len());

    // let caps = re.captures(&*args.time).unwrap();
    for cap in re.captures_iter(&*args.time) {
        println!("{}", cap.get(4).unwrap().as_str());
    };

}
fn turn_time_to_seconds(time: String) -> i32 {
    unimplemented!()
}


