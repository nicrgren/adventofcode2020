use std::{fs::File, io::Read};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

pub type Error = Box<dyn std::error::Error + 'static>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn read_input(input_file: &str) -> Result<String> {
    let mut f = File::open(format!("inputs/{}", input_file))?;
    let mut buf = String::new();
    f.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> Result<()> {
    let days = &[
        day01::solve,
        day02::solve,
        day03::solve,
        day04::solve,
        day05::solve,
        day06::solve,
        day07::solve,
        day08::solve,
        day09::solve,
        day10::solve,
        day11::solve,
        day12::solve,
        day13::solve,
    ];

    match std::env::args().nth(1).map(|s| s.parse::<usize>()) {
        Some(Ok(0)) => println!("There's day 0"),
        Some(Ok(n)) if n <= days.len() => days[n - 1]()?,
        Some(Ok(n)) => println!("Day {} is not implemented yet.", n),
        Some(Err(err)) => println!("Invalid day argument: {}.", err),
        None => {
            for day in days {
                day()?;
            }
        }
    }

    Ok(())
}
