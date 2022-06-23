mod command_line;

use clap::{Command, Parser};
use regex::Regex;
use std::time;

const DATE_PATTERN: &str = r"((?P<years>\d+)y)?\
(?P<days>\d+)d)?\
((?P<hours>\d+)h)?\
((?P<minutes>\d+)m)?\
((?P<seconds>\d+)s)?";
use std::thread;
use std::time::Duration;

struct Time {
    years: u64,
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
    total: Duration,
}

fn main() {
    let start = time::Instant::now();
    let args: command_line::Args = command_line::Args::parse();
    println!("{}", DATE_PATTERN);

    let user_time = parse_input(args.time);
    let duration = user_time.total;

    println!("{:?}", user_time.total);
    println!("{:?}", start);
    let first_run: bool = false;
    loop {
        let loop_start = time::Instant::now();
        println!("How many ms have elapsed since start of main: {}", start.elapsed().as_millis());
        let elapsed = start.elapsed();

        if duration < elapsed {
            println!("BOOM | TIME HAS RUN OUT");
            break;
        }

        let remain = duration.as_secs() - elapsed.as_secs();
        let loop_elapsed = loop_start.elapsed();
        let time_left = match first_run {
            true => start.elapsed().as_micros(),
            false => 1_000_000,
        };
        if loop_elapsed.as_millis() < time_left {
            println!("MICROS: {}", loop_elapsed.as_micros());

            let loop_remain: u128 = time_left - loop_elapsed.as_micros();
            println!("REMAIN: {}", loop_remain);

            thread::sleep(Duration::from_micros(remain));

        };

        println!("Time Left: {:?}", remain);
        thread::sleep(Duration::from_millis(1000));
    }
}

fn parse_input(duration: String) -> Time {
    let re = Regex::new(r"((?P<years>\d+)y)?((?P<days>\d+)d)?((?P<hours>\d+)h)?((?P<minutes>\d+)m)?((?P<seconds>\d+)s)?").unwrap();
    //let re = Regex::new(DATE_PATTERN).unwrap();
    let caps = re.captures(&*duration).unwrap();

    let y: u64 = caps.name("years").map_or(0, |m| m.as_str().parse().unwrap());
    let d: u64 = caps.name("days").map_or(0, |m| m.as_str().parse().unwrap());
    let h: u64 = caps.name("hours").map_or(0, |m| m.as_str().parse().unwrap());
    let m: u64 = caps.name("minutes").map_or(0, |m| m.as_str().parse().unwrap());
    let s: u64 = caps.name("seconds").map_or(0, |m| m.as_str().parse().unwrap());

    let total_secs = Duration::new(36000 * h + 60 * m + s, 0);
    Time {
        years: y,
        days: d,
        hours: h,
        minutes: m,
        seconds: s,
        total: total_secs,
    }
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


