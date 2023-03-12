use std::io;

type Reader = Box<dyn io::BufRead>;
type Int = usize;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    let num_test_cases = read_int(reader_ref);
    for _ in 0..num_test_cases {
        println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
    }
}

struct TestCase {
    k: Int,
    x: Int,
}

struct Answer {
    num_messages: Int,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let numbers = read_array(reader);
        let k = numbers[0];
        let x = numbers[1];
        Self { k, x }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        self.num_messages.to_string()
    }
}

fn solve(case: TestCase) -> Answer {
    let predicate = build_predicate(case.x, case.k);
    let num_messages = binary_search(0, (2 * case.k) - 1, predicate);

    Answer { num_messages }
}

fn build_predicate(x: Int, k: Int) -> Box<dyn Fn(Int) -> bool> {
    Box::new(move |n_messages| num_emotes_from_messages(n_messages, k) < x)
}

fn binary_search(min: Int, max: Int, predicate: Box<dyn Fn(Int) -> bool>) -> Int {
    let mid = (min + max) / 2;
    if mid == min {
        if predicate(min) {
            return max;
        }
        return min;
    }
    if predicate(mid) {
        binary_search(mid, max, predicate)
    } else {
        binary_search(min, mid, predicate)
    }
}

fn num_emotes_from_messages(n_messages: Int, k: Int) -> Int {
    if n_messages <= k {
        return sum_to_n(n_messages);
    } else {
        let remainder = sum_to_n(((2 * k) - 1) - n_messages);
        let total = 2 * sum_to_n(k - 1) + k;
        return total - remainder;
    }
}

fn sum_to_n(n: Int) -> Int {
    (n * (n + 1)) / 2
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

    use crate::{read_int, solve, TestCase};

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
            let n = read_int(&mut reader);
            let actual = (0..n)
                .map(|_| solve(TestCase::from_input(&mut reader)).num_messages)
                .collect::<Vec<_>>();
            assert_eq!(actual, test_case_output(&each));
        }
    }

    fn test_case_reader(test_case: &Test) -> Box<dyn BufRead> {
        Box::new(io::BufReader::new(
            fs::File::open(&test_case.input_file).unwrap(),
        ))
    }

    fn test_case_output(test_case: &Test) -> Vec<usize> {
        let file = fs::File::open(&test_case.output_file).unwrap();
        let buf_reader = BufReader::new(file);
        buf_reader
            .lines()
            .map(|line| line.expect("failed to read line"))
            .map(|line| line.trim().parse().expect("expect an integer"))
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
