use clap::{command, Parser};
use regex::Regex;
use std::io::{self, prelude::*};
use std::{fs::File, io::BufReader};

#[derive(Parser, Debug)]
#[command(name = "grep-lite")]
#[command(version = "0.1")]
#[command(about = "searches for patterns", long_about = None)]
struct Args {
    /// The pattern to search for
    #[arg(value_name = "pattern")]
    pattern: String,

    /// File to search
    #[arg(value_name = "input", default_value = "-")]
    input: String,
}

fn process_lines<T: BufRead + Sized>(reader: T, re: Regex) {
    for line_ in reader.lines() {
        let line = line_.unwrap();
        let contains_substring = re.find(&line);
        match contains_substring {
            Some(_) => println!("{}", line),
            None => (),
        }
    }
}

fn main() {
    let args = Args::parse();

    let re = Regex::new(&args.pattern).unwrap();

    let input = &args.input;
    match input.as_str() {
        "-" => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            process_lines(reader, re);
        }
        _ => {
            let f = File::open(input).unwrap();
            let reader = BufReader::new(f);
            process_lines(reader, re);
        }
    }
}
