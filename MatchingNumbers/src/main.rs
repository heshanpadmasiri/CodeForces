use std::{fmt::Display, io};

type Reader = Box<dyn io::BufRead>;
type Int = i64;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    let t = read_int(reader_ref);
    (0..t)
        .map(|_| solve(TestCase::from_input(reader_ref)))
        .for_each(|answer| {
            print!("{}", answer);
        });
}

struct TestCase {
    n: i64,
}

struct Answer {
    answer: Option<Vec<(Int, Int)>>,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let n = read_int(reader);
        Self { n }
    }
}

impl Display for Answer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.answer {
            Some(answer) => {
                writeln!(f, "YES")?;
                for each in answer {
                    writeln!(f, "{} {}", each.0, each.1)?;
                }
            }
            None => {
                writeln!(f, "NO")?;
            }
        }
        Ok(())
    }
}

fn solve(case: TestCase) -> Answer {
    let n = case.n;
    if n % 2 == 0 {
        return Answer { answer: None };
    }
    let mut lhs = n;
    let mut rhs = lhs + 1;
    let mut answer = Vec::new();
    while lhs > 0 {
        answer.push((lhs, rhs));
        lhs -= 2;
        rhs += 1;
    }
    lhs = 2;
    let rhs_min = rhs-1;
    rhs = 2 * n;
    while rhs > rhs_min {
        answer.push((lhs, rhs));
        lhs += 2;
        rhs -= 1;
    }
    Answer {
        answer: Some(answer),
    }
}

fn read_int(reader: &mut Reader) -> Int {
    let line = read_line(reader);
    return line.trim().parse().expect("expect an integer");
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
            .map(|line| line.expect("failed to read line") + "\n")
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
