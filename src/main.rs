use clap::Parser;
use colored::Colorize;
use regex::Regex;
use std::cmp::min;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Read};

#[derive(Parser, Debug)]
// #[command(version, about, long_about = None)]
struct Args {
    pattern: String,

    #[arg(short, long, default_value_t = 2)]
    context_lines: usize,

    #[arg(short, long)]
    file: Option<String>,
}

fn process_lines<R: Read>(reader: R) -> Vec<(usize, String)> {
    let buffer = BufReader::new(reader);
    buffer
        .lines()
        .enumerate()
        .map(|(i, r)| (i, r.unwrap()))
        .collect::<Vec<(usize, String)>>()
}

fn main() {
    let args = Args::parse();
    let pattern = args.pattern;
    let context_lines = args.context_lines;

    let needle = Regex::new(&pattern).unwrap();
    let search_vec: Vec<(usize, String)> = match args.file {
        None => process_lines(io::stdin()),
        Some(i) => process_lines(File::open(i).unwrap()),
    };

    let mut ctx: Vec<&[(usize, String)]> = vec![];
    let num_lines = search_vec.len();
    println!("Searching for {:?} in text..\n", pattern);

    for (i, line) in search_vec.iter() {
        if needle.find(line).is_some() {
            ctx.push(
                &search_vec[i.saturating_sub(context_lines)
                    ..min(i.saturating_add(context_lines) + 1, num_lines)],
            );
        }
    }
    for ls in ctx.iter() {
        for (j, (i, l)) in ls.iter().enumerate() {
            if j == context_lines {
                println!("{}: {}", i.to_string().red().bold(), l);
            } else {
                println!("{}: {}", i, l);
            }
        }
        println!("\n")
    }
}
