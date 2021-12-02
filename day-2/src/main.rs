use std::fs;
use std::path;
use std::process;
use structopt::StructOpt;


#[derive(StructOpt)]
struct Config {
    /// The path to the input file
    #[structopt(parse(from_os_str))]
    input_path: path::PathBuf,

    /// Switch to aim-based command execution
    #[structopt(long)]
    aim: bool,
}

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Command {
    fn new(s: &str) -> Result<Command, &str> {
        let (command_str, value_str) = match s.split_once(' ') {
            Some(t) => t,
            None => return Err("Failed to parse command"),
        };

        let value = match value_str.parse::<i32>() {
            Ok(value) => value,
            Err(_) => return Err("Failed to parse value"),
        };

        let command = match command_str {
            "forward" => Command::Forward(value),
            "down"    => Command::Down(value),
            "up"      => Command::Up(value),
            _         => return Err("Unrecognized command"),
        };

        Ok(command)
    }
}

struct State {
    depth: i32,
    hposition: i32,
    aim: i32,
}

impl State {
    fn new() -> State {
        State {
            depth: 0,
            hposition: 0,
            aim: 0
        }
    }
}

fn naive_update(state: &mut State, c: Command) {
    match c {
        Command::Forward(value) => state.hposition += value,
        Command::Up(value) => state.depth -= value,
        Command::Down(value) => state.depth += value,
    };
}

fn aim_update(state: &mut State, c: Command) {
    match c {
        Command::Up(value) => state.aim -= value,
        Command::Down(value) => state.aim += value,
        Command::Forward(value) => {
            state.hposition += value;
            state.depth += state.aim * value;
        }
    };
}

fn main() {
    let config = Config::from_args();

    let update_fn = if config.aim {
        aim_update
    } else {
        naive_update
    };

    let contents = fs::read_to_string(&config.input_path).unwrap_or_else(|err| {
        eprintln!("Error reading the input file: {}", err);
        process::exit(1);
    });

    let commands: Vec<Command> =
        contents
        .lines()
        .map(|line| Command::new(line).unwrap())
        .collect();

    let mut state = State::new();

    for command in commands {
        update_fn(&mut state, command);
    }

    println!("{}", state.hposition * state.depth);
}
