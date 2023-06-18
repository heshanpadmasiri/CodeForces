use std::{collections::HashMap, io};

type Reader = Box<dyn io::BufRead>;
type Int = i64;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
}

struct TestCase {
    message: String,
}

struct Answer {
    n_messages: u64,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let message = read_line(reader);
        Self { message }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        self.n_messages.to_string()
    }
}

fn solve(case: TestCase) -> Answer {
    let chars = case.message.chars().collect::<Vec<_>>();
    let mut dp = HashMap::new();
    let n_messages = possible_cases(&chars, 0, &mut dp);
    Answer { n_messages }
}

const MOD: u64 = 1_000_000_007;

type DP = HashMap<usize, u64>;

// Number of messages starting at index
fn possible_cases(chars: &[char], index: usize, dp: &mut DP) -> u64 {
    if index >= chars.len() {
        return 1;
    }
    if dp.contains_key(&index) {
        return *dp.get(&index).unwrap();
    }
    let result = match chars[index] {
        'w' | 'm' => 0,
        'u' | 'n' => {
            match peek(chars, index + 1) {
                Some(c) => {
                    if c == chars[index] {
                        // We have uu or nn
                        (possible_cases(chars, index + 2, dp) % MOD)
                            + (possible_cases(chars, index + 1, dp) % MOD)
                    } else {
                        possible_cases(chars, index + 1, dp)
                    }
                }
                None => possible_cases(chars, index + 1, dp),
            }
        }
        _ => possible_cases(chars, index + 1, dp),
    } % MOD;
    // assert no double counting
    assert!(!dp.contains_key(&index));
    dp.insert(index, result);
    result
}

fn peek(chars: &[char], index: usize) -> Option<char> {
    if index >= chars.len() {
        return None;
    }
    Some(chars[index])
}

fn read_int(reader: &mut Reader) -> Int {
    let line = read_line(reader);
    return line.trim().parse().expect("expect an integer");
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
