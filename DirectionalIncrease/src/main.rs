use std::io;

type Reader = Box<dyn io::BufRead>;
type Int = i64;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    let n = read_int(reader_ref);
    let solutions: Vec<String> = (0..n).map(|_| solve(TestCase::from_input(reader_ref))).map(|x| x.to_string()).collect();
    for each in solutions {
        println!("{}", each);
    }
}

struct TestCase {
    numbers: Vec<Int>,
}

#[derive(Debug, PartialEq)]
enum Answer {
    YES,
    NO
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let _ = read_int(reader);
        let numbers = read_array(reader);
        Self { numbers }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        match self {
            Answer::YES => "Yes".to_string(),
            Answer::NO => "No".to_string(),
        }
    }
}

struct State {
    x_sum: Int,
    y_sum: Int,
    x_prev: Int,
}

fn solve(case: TestCase) -> Answer {
    let x_0 = case.numbers[0];
    let result = case.numbers.iter().skip(1).try_fold(State { x_sum: x_0, y_sum: 0, x_prev: x_0 }, |State {x_sum, y_sum, x_prev }, a_cur| {
        if x_prev == 0 && *a_cur != 0 {
            return None;
        }
        let x_cur = a_cur + x_prev;
        let y_cur = x_prev;
        if x_cur >= 0 && y_cur >= 0 {
            return Some(State {
                x_sum: x_sum + x_cur,
                y_sum: y_sum + y_cur,
                x_prev: x_cur,
            });
        }
        None
    });
    if let Some(State {x_sum, y_sum, x_prev:_}) = result {
        if x_sum == y_sum {
            return Answer::YES;
        }
    }
    Answer::NO
}

fn read_int(reader: &mut Reader) -> Int {
    let line = read_line(reader);
    return line.trim().parse().expect("expect an integer");
}

fn read_array(reader: &mut Reader) -> Vec<Int> {
    let line = read_line(reader);
    return line.trim().split(' ').flat_map(str::parse::<Int>).collect::<Vec<_>>();
}

fn std_reader() -> Reader {
    Box::new(io::BufReader::new(io::stdin()))
}

fn read_line(input_reader: &mut Reader) -> String {
    let mut line = String::new();
    input_reader.read_line(&mut line).expect("expect input line");
    line
}

#[cfg(test)]
mod test {
    use std::fs;
    use std::io::{self, BufRead, BufReader};
    use std::path::Path;

    use crate::{solve, TestCase, read_int};

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
            let reader_ref = &mut reader;
            let n = read_int(reader_ref);
            let actual: Vec<String> = (0..n).map(|_| solve(TestCase::from_input(reader_ref))).map(|x| x.to_string()).collect();
            assert_eq!(actual, test_case_output(&each));
        }
    }

    fn test_case_reader(test_case: &Test) -> Box<dyn BufRead> {
        Box::new(io::BufReader::new(
            fs::File::open(&test_case.input_file).unwrap(),
        ))
    }

    fn test_case_output(test_case: &Test) -> Vec<String> {
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
