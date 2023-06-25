use std::io;

type Reader = Box<dyn io::BufRead>;
type Int = i64;
#[derive(Clone, Debug)]
enum Move {
    Rock,
    Paper,
    Scissors,
}
impl Move {
    fn from_str(s: &str) -> Self {
        match s {
            "R" => Move::Rock,
            "P" => Move::Paper,
            "S" => Move::Scissors,
            _ => panic!("invalid move: {}", s),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Move::Rock => 'R',
            Move::Paper => 'P',
            Move::Scissors => 'S',
        }
    }
}

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    let t = read_int(reader_ref);
    for _ in 0..t {
        println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
    }
}

struct TestCase {
    moves: Vec<Move>,
}

struct Answer {
    moves: Vec<Move>,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let moves = read_moves(reader);
        Self { moves }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        self.moves.iter().map(|m| m.to_char()).collect::<String>()
    }
}

fn solve(case: TestCase) -> Answer {
    let mut rock_count = 0;
    let mut paper_count = 0;
    let mut scissors_count = 0;
    for m in case.moves.iter() {
        match best_response(m) {
            Move::Rock => rock_count += 1,
            Move::Paper => paper_count += 1,
            Move::Scissors => scissors_count += 1,
        }
    }
    // dbg!(rock_count, paper_count, scissors_count);
    let best_move = best_average_response(rock_count, paper_count, scissors_count);
    // dbg!(&best_move);
    let moves = vec![best_move; case.moves.len()];
    Answer { moves }
}

fn best_average_response(rock_count: usize, paper_count: usize, scissors_count: usize) -> Move {
    if rock_count >= paper_count && rock_count >= scissors_count {
        Move::Rock
    } else if paper_count >= rock_count && paper_count >= scissors_count {
        Move::Paper
    } else {
        Move::Scissors
    }
}

fn best_response(m: &Move) -> Move {
    match m {
        Move::Rock => Move::Paper,
        Move::Paper => Move::Scissors,
        Move::Scissors => Move::Rock,
    }
}


fn read_int(reader: &mut Reader) -> Int {
    let line = read_line(reader);
    return line.trim().parse().expect("expect an integer");
}

fn read_moves(reader: &mut Reader) -> Vec<Move> {
    let line = read_line(reader);
    return line
        .trim()
        .chars()
        .map(|c| Move::from_str(&c.to_string()))
        .collect::<Vec<_>>();
}

fn read_array(reader: &mut Reader) -> Vec<Int> {
    let line = read_line(reader);
    return line
        .trim()
        .split(' ')
        .flat_map(str::parse::<Int>)
        .collect::<Vec<_>>();
}

fn std_reader() -> Reader {
    Box::new(io::BufReader::new(io::stdin()))
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

    use crate::{solve, TestCase};

    #[derive(Debug)]
    struct Test {
        input_file: std::path::PathBuf,
        output_file: std::path::PathBuf,
    }

    #[test]
    fn test() {
        for each in get_test_cases() {
            println!("TEST_CASE: {:?}", each);
            let mut reader = test_case_reader(&each);
            let actual = solve(TestCase::from_input(&mut reader)).to_string();
            assert_eq!(actual, test_case_output(&each));
        }
    }

    fn test_case_reader(test_case: &Test) -> Box<dyn BufRead> {
        Box::new(io::BufReader::new(
            fs::File::open(&test_case.input_file).unwrap(),
        ))
    }

    fn test_case_output(test_case: &Test) -> String {
        let file = fs::File::open(&test_case.output_file).unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|line| line.expect("failed to read line"))
            .collect()
    }

    fn get_test_cases() -> Vec<Test> {
        println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        let mut test_cases = Vec::new();
        for each in fs::read_dir("./input").unwrap() {
            let input_file = each.unwrap().path();
            let output_file_name = input_file.file_stem().unwrap().to_str().unwrap();
            let output_file =
                Path::new(&format!("./output/{}.out", output_file_name)).to_path_buf();
            assert!(output_file.exists());
            test_cases.push(Test {
                input_file,
                output_file,
            });
        }
        test_cases
    }
}
