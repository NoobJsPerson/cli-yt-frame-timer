#![allow(unused)]

use clap::Parser;
use clap::builder::Str;
use std::fmt::format;
use std::fs::read_to_string;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    start: PathBuf,
    end: PathBuf,
	framerate: f64
}
fn format_time(time: f64) -> String {
	let hours: f64 = (time / 3600 as f64).floor();
	let minutes: f64 = ((time % 3600 as f64) / 60 as f64).floor();
	let seconds: f64 = ((time % 60 as f64) * 1000 as f64).floor() / 1000 as f64;
	let hours_str: String;
	let minutes_str: String;
	let seconds_str: String;
	if hours < 10 as f64 {
		hours_str = "0".to_owned();
	} else {
		hours_str = "".to_owned();
	}
	if minutes < 10 as f64 {
		minutes_str = "0".to_owned();
	} else {
		minutes_str = "".to_owned();
	}
	if seconds < 10 as f64 {
		seconds_str = "0".to_owned();
	} else {
		seconds_str = "".to_owned();
	}
	return format!("{}{}:{}{}:{}{}",hours_str,hours,minutes_str,minutes,seconds_str,seconds);
}
fn main() {
    let args = Cli::parse();
    let start = args.start.display();
    let end = args.end.display();
	let framerate = args.framerate;
    let start_content = read_to_string(&args.start).expect("could not read file");
    let end_content = read_to_string(&args.end).expect("could not read file");
	let mut start_num: f64 = -1 as f64;
	let mut end_num: f64 = -1 as f64;

    for line in start_content.lines() {
        if line.starts_with("	\"lc") {
			let mut parts = line.split("\"");
			let first = parts.nth(3).unwrap();
			start_num = first.parse::<f64>().unwrap();
			break;
        }
    }
	if(start_num == -1 as f64) {
		panic!("couldn't find start (maybe the file you specified wasn't a valid yt json file)");
	};
	for line in end_content.lines() {
        if line.starts_with("	\"lc") {
			let mut parts = line.split("\"");
			let first = parts.nth(3).unwrap();
			end_num = first.parse::<f64>().unwrap();
			break;
        }
    }
	if(end_num == -1 as f64) {
		panic!("couldn't find end (maybe the file you specified wasn't a valid yt json file)");
	};
	let final_num = ((end_num - start_num) * framerate).floor() / framerate;
	println!("{}",format_time(final_num));
}
