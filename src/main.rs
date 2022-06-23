mod command_line;

use clap::{Parser};
use regex::Regex;
use std::time;

const DATE_PATTERN: &str = r"((?P<years>\d+)y)?\
(?P<days>\d+)d)?\
((?P<hours>\d+)h)?\
((?P<minutes>\d+)m)?\
((?P<seconds>\d+)s)?";

use std::thread;
use std::time::Duration;

#[allow(unused_variables)]
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
    //println!("{}", DATE_PATTERN);

    let user_time = parse_input(args.time);
    let duration = user_time.total;

    println!("{:?}", user_time.total);

    let mut first_run: bool = true;
    'countdown: loop {
        let loop_start = time::Instant::now();
        //println!("How many ms have elapsed since start of main: {}", start.elapsed().as_millis());
        let elapsed = start.elapsed();

        if duration < elapsed {
            println!("BOOM | TIME HAS RUN OUT");
            break 'countdown;
        }

        let remain = duration.as_secs() - elapsed.as_secs();
        let loop_elapsed = loop_start.elapsed();
        let time_left = match first_run {
            true => start.elapsed().as_micros(),
            false => 1_000_000,
        };

        if first_run { first_run = false;}

        //println!("Checking first_run condition: {}", time_left);
        if loop_elapsed.as_micros() < time_left {
            //println!("MICROS: {}", loop_elapsed.as_micros());

            let loop_remain: u128 = time_left - loop_elapsed.as_micros();
            //println!("REMAIN: {}", loop_remain);

            // Sleeps main thread until the second is finished, then it prints out
            thread::sleep(Duration::from_micros(remain));
        };

        println!("Time Left: {:?}", remain);
        thread::sleep(Duration::from_millis(1000));
    }
}

// TODO: Use constant raw string literal for pattern matching
fn parse_input(duration: String) -> Time {
    let re = Regex::new(r"((?P<years>\d+)y)?((?P<days>\d+)d)?((?P<hours>\d+)h)?((?P<minutes>\d+)m)?((?P<seconds>\d+)s)?").unwrap();
    //let re = Regex::new(DATE_PATTERN).unwrap();
    let caps = re.captures(&*duration).unwrap();

    // We have to parse the captures as strings for their integer counterpart
    let y: u64 = caps.name("years")
        .map_or(0, |m| m.as_str().parse().unwrap());
    let d: u64 = caps.name("days")
        .map_or(0, |m| m.as_str().parse().unwrap());
    let h: u64 = caps.name("hours")
        .map_or(0, |m| m.as_str().parse().unwrap());
    let m: u64 = caps.name("minutes")
        .map_or(0, |m| m.as_str().parse().unwrap());
    let s: u64 = caps.name("seconds")
        .map_or(0, |m| m.as_str().parse().unwrap());

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

// TODO: Turn remaining time to printable formatted string
#[allow(dead_code)]
fn turn_time_to_string(time: Duration) -> String {
    unimplemented!()
}


