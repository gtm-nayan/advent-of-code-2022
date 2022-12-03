/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */

use std::fs::File;
use std::io::Write;
use std::process;
use std::{io, process::Command};

struct Args {
    day: u8,
    year: Option<i16>,
}

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = pico_args::Arguments::from_env();
    Ok(Args {
        day: args.free_from_str()?,
        year: args.opt_value_from_str(["-y", "--year"])?,
    })
}

fn main() {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Failed to process arguments: {}", e);
            process::exit(1);
        }
    };

    let day = args.day;
    let year = args.year.unwrap_or_else(|| {
        use chrono::Datelike;
        chrono::offset::Utc::now().year() as _
    });

    let curl_output = Command::new("curl")
        .args([
            "-H",
            "@.session",
            &format!("https://adventofcode.com/{year}/day/{day}/input"),
        ])
        .output()
        .expect("Failed to spawn curl.");

    File::options()
        .write(true)
        .open(format!("./src/inputs/{day:02}.txt"))
        .unwrap()
        .write_all(&curl_output.stdout)
        .expect("could not write cmd stdout to pipe.");

    io::stderr()
        .write_all(&curl_output.stderr)
        .expect("could not write cmd stderr to pipe.");
}
