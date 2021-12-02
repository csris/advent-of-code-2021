use itertools::Itertools;
use std::fs;
use std::path;
use std::process;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Config {
    /// The path to the input file
    #[structopt(parse(from_os_str))]
    input_path: path::PathBuf,

    /// The sliding window size for the sum filter
    #[structopt(long, default_value = "1")]
    sliding_window_size: u32,
}

fn sum_filter<I>(iter: I) -> impl Iterator<Item = u32>
where 
    I: Iterator<Item = u32>
{
    iter
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
}

fn count_increases(iter: &mut dyn Iterator<Item = u32>) -> usize
{
    iter
        .tuple_windows()
        .filter(|(a, b)| b > a)
        .count()
}

fn main() {
    let config = Config::from_args();

    let contents = fs::read_to_string(&config.input_path).unwrap_or_else(|err| {
        eprintln!("Error reading the input file: {}", err);
        process::exit(1);
    });

    let mut measurements: Box<dyn Iterator<Item = u32>> = 
        Box::new(
            contents
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
        );

    if config.sliding_window_size > 1 {
        measurements = Box::new(sum_filter(measurements));
    };
    
    println!("{}", count_increases(&mut measurements));
}
