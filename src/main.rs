mod command_line;

use figlet_rs::FIGfont;
use std::io::stdout;
use clap::{Parser};
use regex::Regex;
use std::{thread, time::Duration, time};
use clap::ArgAction::Set;
use crossterm::{execute,
                style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
                ExecutableCommand,
                Result, event, queue, terminal};

const DATE_PATTERN: &str = r"((?P<years>\d+)y)?\
(?P<days>\d+)d)?\
((?P<hours>\d+)h)?\
((?P<minutes>\d+)m)?\
((?P<seconds>\d+)s)?";

#[allow(unused_variables)]
struct Time {
    years: u64,
    days: u64,
    hours: u64,
    minutes: u64,
    seconds: u64,
    total: Duration,
}

fn main() -> Result<()> {
    let start = time::Instant::now();
    let args: command_line::Args = command_line::Args::parse();
    let user_time = parse_input(args.time);
    let duration = user_time.total;
    let mut first_run: bool = true;
    let standard_font = FIGfont::standand().unwrap(); 
    stdout().execute(SetForegroundColor(Color::Green))?; 
    stdout().execute(SetBackgroundColor(Color::Black))?;

    println!("Counting down from: {:?}", user_time.total);

    stdout().execute(SetForegroundColor(Color::Red))?;

    'countdown: loop {
        let loop_start = time::Instant::now();
        let elapsed = start.elapsed();

        if duration < elapsed {
            exit_program();
            break 'countdown;
        }

        let remain = duration.as_secs() - elapsed.as_secs();

        let time_left = match first_run {
            true => 1_000_000 - start.elapsed().as_micros(),
            false => 1_000_000,
        };

        if first_run { first_run = false;}

        execute!(stdout(), terminal::Clear(terminal::ClearType::FromCursorUp))
            .expect("Failed to clear");

        let loop_elapsed = loop_start.elapsed();
        //println!("Checking first_run condition: {}", time_left);
        let mut loop_remain: u128 = time_left;

        if loop_elapsed.as_micros() < time_left {
            //println!("MICROS: {}", loop_elapsed.as_micros());
            loop_remain = time_left - loop_elapsed.as_micros();
            // println!("LOOP_REMAIN: {}", loop_remain);
            // println!("Remain: {}", remain);
        };

        output_timer(remain, &standard_font);
        // Sleeps main thread until the second is finished, then it prints out
        thread::sleep(Duration::from_micros(loop_remain as u64));
        // thread::sleep(Duration::from_millis(1000));
    }
    Ok(())
}

fn output_timer(remain: u64, font: &FIGfont) {
    let stringer = font.convert(remain.to_string().as_str()).expect("failed");

    println!("{}",  stringer);
}
fn exit_program()  {
    stdout().execute(terminal::Clear(terminal::ClearType::All)
    ).expect("Failed to clear terminal");
    println!("BOOM | TIME IS UP");
    stdout().execute(ResetColor)
        .expect("Failed to reset colors");
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


