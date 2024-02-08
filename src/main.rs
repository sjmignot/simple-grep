use std::cmp::min;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use regex::Regex;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    pattern: String,

    #[arg(short, long, default_value_t = 2)]
    context_lines: usize,

    #[arg(short, long, default_value_t = String::from("-"))]
    file: String
}

fn main() {
    let args = Args::parse();
    let pattern = args.pattern;
    let context_lines = args.context_lines;
    let input = args.file;

    let needle = Regex::new(&pattern).unwrap();
    let f = File::open(input).unwrap();
    let reader = BufReader::new(f);

    let mut ctx: Vec<&[(usize, String)]> = vec!{};
    let search_vec = reader.lines().enumerate().map(|(i, r)| {(i, r.unwrap())}).collect::<Vec<(usize, String)>>();
    let num_lines = search_vec.len();

    for (i, line) in search_vec.iter() {
        match needle.find(line) {
            Some(_) => {
                ctx.push(
                    &search_vec[i.saturating_sub(context_lines)..min(i.saturating_add(context_lines)+1, num_lines)]
                );
            },
            None => ()
        }
    }
    for ls in ctx.iter() {
        for (i, l) in ls.iter() {
            println!("{i}: {l}");
        }
    }
}
