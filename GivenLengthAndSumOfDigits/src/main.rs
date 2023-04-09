use core::num;
use std::io;

type Reader = Box<dyn io::BufRead>;
type Int = usize;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
}

struct TestCase {
    length: usize,
    sum: usize
}

struct Answer {
    largest: Option<String>,
    smallest: Option<String>
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let data = read_array(reader);
        let length = data[0];
        let sum = data[1];
        Self { length, sum }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        format!("{} {}", optional_int_to_string(&self.smallest), optional_int_to_string(&self.largest))
    }
}

fn optional_int_to_string(value: &Option<String>) -> String {
    match value {
        Some(value) => value.to_owned(),
        None => "-1".to_string(),
    }
}

fn solve(case: TestCase) -> Answer {
    let length = case.length;
    let sum = case.sum;
    Answer { largest: largest_possible(length, sum), smallest: smallest_possible(length, sum) }
}

fn smallest_possible(length: usize, sum: usize) -> Option<String> {
    let mut digits = Vec::with_capacity(length);
    let length = length as i64;
    let mut remainder = sum as i64;
    for i in 0..length {
        for digit in 0..10 {
            if length != 1 && i == 0 && digit == 0 {
                // No leading zeros
                continue;
            }
            let remaining_length = length - (i+1);
            let remaining_sum = remainder - digit;
            if remaining_sum < 0 || (remaining_length == 0 && digit != remainder) {
                // No leading zeros
                continue;
            }
            if is_possible(remaining_length as usize, remaining_sum as usize) {
                digits.push(digit);
                remainder -= digit;
                break;
            }
        }
    }
    if digits.len() == length as usize {
        Some(digits.into_iter().map(|each| each.to_string()).collect())
    }
    else {
        None
    }
}

fn largest_possible(length: usize, sum: usize) -> Option<String> {
    let mut digits = Vec::with_capacity(length);
    let length = length as i64;
    let mut remainder = sum as i64;
    for i in 0..length {
        for digit in (0..=9).rev() {
            if length != 1 && i == 0 && digit == 0 {
                // No leading zeros
                continue;
            }
            let remaining_length = length - (i+1);
            let remaining_sum = remainder - digit;
            if remaining_sum < 0 || (remaining_length == 0 && digit != remainder) {
                // No leading zeros
                continue;
            }
            if is_possible(remaining_length as usize, remaining_sum as usize) {
                digits.push(digit);
                remainder -= digit;
                break;
            }
        }
    }
    if digits.len() == length as usize {
        Some(digits.into_iter().map(|each| each.to_string()).collect())
    }
    else {
        None
    }
}

fn is_possible(length: usize, sum: usize) -> bool {
    length >= 0 && sum <= 9*length
}

fn is_valid(number: Int, sum: Int) -> bool {
    let mut actual = 0;
    let mut number = number;
    while number > 0 {
        actual += number % 10;
        if actual > sum {
            break;
        }
        number /= 10;
    }
    return actual == sum;
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
