mod command_line;

use figlet_rs::FIGfont;
use clap::{Parser, ArgAction::Set};
use regex::Regex;
use std::{io::stdout, thread, time::Duration, time};
use std::fmt::format;
use std::str::FromStr;
use crossterm::{execute,
                style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
                ExecutableCommand,
                Result, event, queue, terminal};

const DATE_PATTERN: &str =
    r"((?P<years>\d+)y)?((?P<days>\d+)d)?((?P<hours>\d+)h)?((?P<minutes>\d+)m)?((?P<seconds>\d+)s)?";

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
    let standard_font = FIGfont::standand()
        .expect("Failed to create figfont");

    stdout().execute(SetBackgroundColor(
        Color::from_str(&*args.back_color).unwrap()))?;
    stdout().execute(SetForegroundColor(
        Color::from_str(&*args.text_color).unwrap()))?;

    'countdown: loop {
        let loop_start = time::Instant::now();
        let elapsed = start.elapsed();

        if duration < elapsed {
            exit_program(&standard_font);
            break 'countdown;
        }

        let remain = duration.as_secs() - elapsed.as_secs();

        let time_left = match first_run {
            true => 1_000_000_000 - start.elapsed().as_nanos(),
            false => 1_000_000_000,
        };

        if first_run { first_run = false;}

        execute!(stdout(), terminal::Clear(terminal::ClearType::FromCursorUp))
            .expect("Failed to clear");

        let loop_elapsed = loop_start.elapsed();
        //println!("Checking first_run condition: {}", time_left);
        let mut loop_remain: u128 = time_left ;

        if loop_elapsed.as_nanos() < time_left {
            //println!("MICROS: {}", loop_elapsed.as_micros());
            loop_remain = time_left - loop_elapsed.as_nanos();
            // println!("LOOP_REMAIN: {}", loop_remain);
            // println!("Remain: {}", remain);
        };
        let str = from_time(remain);
        output_timer(str, &standard_font);

        // Sleeps main thread until the second is finished, then it prints out
        thread::sleep(Duration::from_nanos(loop_remain as u64));
    }
    Ok(())
}

fn output_timer(str: String, font: &FIGfont) {
    let font_string = font.convert(str.as_str())
        .expect("Failed to format string");

    println!("{}", font_string);
}

fn exit_program(font: &FIGfont)  {
    stdout().execute(terminal::Clear(terminal::ClearType::All))
        .expect("Failed to clear terminal");

    print_fig_string("BOOM", font);

    // println!("BOOM | TIME IS UP");
    stdout().execute(ResetColor)
        .expect("Failed to reset colors");
}

fn print_fig_string(str: &str, font: &FIGfont) {
    let font_string = font.convert(str)
        .expect("Failed to convert str to fig font");

    println!("{}", font_string);
}

fn parse_input(duration: String) -> Time {
    let re = Regex::new(DATE_PATTERN)
        .expect("Failed to create regex");

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

    let total_secs = Duration::new(d * (24 * 3_600) + 3_600 * h + 60 * m + s, 0);
    Time {
        years: y,
        days: d,
        hours: h,
        minutes: m,
        seconds: s,
        total: total_secs,
    }
}

fn from_time(mut remain: u64) -> String {
    let day = remain / (24 * 3600);
    remain %= 24 * 3600;

    let hour = remain / 3600;
    remain %= 3600;

    let minutes = remain / 60;
    remain %= 60;

    let seconds = remain;
    let mut str: String = String::new();

    if day != 0 {
        str.push_str(format!("{}D ", day).as_str());
    }

    if hour != 0 {
        str.push_str(format!("{}H ", hour).as_str());
    }

    if minutes != 0 {
        str.push_str(format!("{}M ", minutes).as_str());
    }

    if seconds != 0 {
        str.push_str(format!("{}S", seconds).as_str());
    }

    str
}
