#![allow(unused)]

use clap::Parser;

#[derive(Parser)]
struct Cli {
	start: std::path::PathBuf,
	end: std::path::PathBuf
}
fn main() {
	let args = Cli::parse();
	let start = args.start.display();
	let end = args.end.display();
	println!("{start}");
	println!("{end}");
	
}
