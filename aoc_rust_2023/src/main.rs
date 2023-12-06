mod helpers;
mod impls;

use clap::Parser;

type AocResult<T> = Result<T, anyhow::Error>;

#[derive(Debug, Parser)]
struct Args {
    #[arg(long)]
    problem: u8,
    #[arg(long)]
    part: u8,
}

pub enum Part {
    One,
    Two,
}

pub enum Solution {
    Str(String),
    Int(i32),
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Str(s) => f.write_str(s),
            Self::Int(i) => f.write_str(&i.to_string()),
        }
    }
}

impl TryFrom<u8> for Part {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Part::One),
            2 => Ok(Part::Two),
            _ => Err(format!("{value} is not a valid part. It must be 1 or 2.")),
        }
    }
}

fn main() -> Result<(), u8> {
    let args = Args::parse();
    println!("Starting AOC 2023 with args: {args:?}");
    let part = match Part::try_from(args.part) {
        Ok(part) => part,
        Err(e) => {
            println!("Error!: {e}");
            return Err(1);
        }
    };
    let lines = match helpers::open_data_file(args.problem) {
        Ok(lines) => lines,
        Err(e) => {
            println!("Failed to open file: {e}");
            return Err(254);
        }
    };
    let result = match args.problem {
        1 => impls::problem_1::solve(part, lines),
        2 => impls::problem_2::solve(part, lines),
        3 => impls::problem_3::solve(part, lines),
        4 => impls::problem_4::solve(part, lines),
        5 => impls::problem_5::solve(part, lines),
        6 => impls::problem_6::solve(part, lines),
        _ => return Err(255),
    };
    match result {
        Ok(solution) => {
            println!(
                "Problem: {}, Part{}, solution: {solution}",
                args.problem, args.part
            );
            Ok(())
        }
        Err(e) => {
            println!("An error occurred solving the problem: {e}");
            Err(2)
        }
    }
}
