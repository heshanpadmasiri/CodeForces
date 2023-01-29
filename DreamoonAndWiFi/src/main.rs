use std::io;

type Reader = Box<dyn io::BufRead>;
type Int = i64;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    println!("{}", solve(reader_ref)[0]);
}

fn solve(input: &mut Reader) -> Vec<String> {
    let send = commands(input);
    let recieved = commands(input);
    let target = possible_outcomes(send)[0];
    let possible_destinations = possible_outcomes(recieved);
    let n_correct = possible_destinations
        .iter()
        .filter(|each| **each == target)
        .count();
    let prob = match n_correct {
        0 => 0.0,
        n => (n as f64) / (possible_destinations.len() as f64)
    };
    vec![format!("{:.12}", prob)]
}

enum Command {
    Plus,
    Minus,
    Unknown,
}

fn std_reader() -> Reader {
    Box::new(io::BufReader::new(io::stdin()))
}

fn commands(input: &mut Reader) -> Vec<Command> {
    read_line(input)
        .trim()
        .chars()
        .map(|each| match each {
            '+' => Command::Plus,
            '-' => Command::Minus,
            '?' => Command::Unknown,
            _ => {
                panic!("unexpected char: {}", each)
            }
        })
        .collect()
}

fn possible_outcomes(commands: Vec<Command>) -> Vec<Int> {
    commands
        .into_iter()
        .fold(vec![0], |current_states, command| {
            next_states(current_states, command)
        })
}

fn next_states(current_states: Vec<Int>, command: Command) -> Vec<Int> {
    current_states
        .into_iter()
        .flat_map(|pos| match command {
            Command::Plus => {
                vec![pos + 1]
            }
            Command::Minus => {
                vec![pos - 1]
            }
            Command::Unknown => {
                vec![pos + 1, pos - 1]
            }
        })
        .collect()
}

fn read_line(input_reader: &mut Reader) -> String {
    let mut line = String::new();
    input_reader
        .read_line(&mut line)
        .expect("expect input line");
    line
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::{self, BufRead, BufReader};
    use std::path::Path;

    use crate::solve;

    #[derive(Debug)]
    struct TestCase {
        input_file: std::path::PathBuf,
        output_file: std::path::PathBuf,
    }

    #[test]
    fn test() {
        for each in get_test_cases() {
            println!("TEST_CASE: {:?}", each);

            let mut reader = test_case_reader(&each);
            let actual = solve(&mut reader);
            assert_eq!(actual, test_case_output(&each));
        }
    }

    fn test_case_reader(test_case: &TestCase) -> Box<dyn BufRead> {
        Box::new(io::BufReader::new(
            fs::File::open(&test_case.input_file).unwrap(),
        ))
    }

    fn test_case_output(test_case: &TestCase) -> Vec<String> {
        let file = fs::File::open(&test_case.output_file).unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|line| line.expect("failed to read line"))
            .collect()
    }

    fn get_test_cases() -> Vec<TestCase> {
        println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        let mut test_cases = Vec::new();
        for each in fs::read_dir("./input").unwrap() {
            let input_file = each.unwrap().path();
            let output_file_name = input_file.file_stem().unwrap().to_str().unwrap();
            let output_file =
                Path::new(&format!("./output/{}.out", output_file_name)).to_path_buf();
            assert!(output_file.exists());
            test_cases.push(TestCase {
                input_file,
                output_file,
            });
        }
        test_cases
    }
}
