use std::{io, collections::BinaryHeap};

type Reader = Box<dyn io::BufRead>;
type Int = usize;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
}

struct TestCase {
    n: Int,
    k: Int,
}

struct Answer {
    sum: Option<Vec<Int>>,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let data = read_array(reader);
        let n = data[0];
        let k = data[1];
        Self { n, k }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        match &self.sum {
            Some(sum) => {
                "YES\n".to_owned()
                    + &sum
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<_>>()
                        .join(" ")
            }
            None => String::from("NO"),
        }
    }
}

fn solve(case: TestCase) -> Answer {
    let ones = usize::count_ones(case.n) as usize;
    let k = case.k;
    if ones > k {
        return Answer { sum: None };
    }
    let mut values = BinaryHeap::new();
    let mut i = 0;
    loop {
        let value = 1 << i;
        if value > case.n {
            break;
        }
        if value & case.n != 0 {
            values.push(value);
        }
        i += 1;
    }
    while values.len() < k {
        let value = values.pop().unwrap();
        if value == 1 {
            return Answer { sum: None };
        }
        values.push(value / 2);
        values.push(value / 2);
    }
    Answer {
        sum: Some(values.into_vec()),
    }
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
