use std::io;

type Integer = usize;
type DigitSet = u16;
fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    for line in solve(reader_ref) {
        println!("{}", line);
    }
}

fn solve(reader: &mut Box<dyn io::BufRead>) -> Vec<String> {
    let t = read_int(reader);
    (0..t)
        .into_iter()
        .map(|_| {
            let data = read_array(reader);
            let a = data[0];
            let k = data[1];
            sequence(a, k)
        })
        .map(|ans| ans.to_string())
        .collect()
}

fn sequence(a: Integer, k: Integer) -> Integer {
    let mut current = a;
    for _ in 0..k - 1 {
        let digits = to_digit_set(current);
        let min = min_digit(digits);
        if min == 0 {
            break;
        }
        let max = max_digit(digits);
        current += min * max;
    }
    current
}

const ALL_SET: DigitSet = (1 << 11) - 1;
fn to_digit_set(val: Integer) -> DigitSet {
    let mut current_val = val;
    let mut digits: DigitSet = 0;
    while current_val > 0 && digits != ALL_SET {
        let digit = current_val % 10;
        digits |= 1 << digit;
        current_val /= 10;
    }
    assert!(digits != 0);
    digits
}

fn min_digit(digits: DigitSet) -> Integer {
    for i in 0..10 {
        if digits & (1 << i) != 0 {
            return i;
        }
    }
    panic!("failed to find min digit");
}

fn max_digit(digits: DigitSet) -> Integer {
    for i in 0..10 {
        let val = 9 - i;
        if digits & (1 << val) != 0 {
            return val;
        }
    }
    panic!("failed to find max digit");
}

fn read_int(reader: &mut Box<dyn io::BufRead>) -> Integer {
    let line = read_line(reader);
    return line.trim().parse().expect("expect an integer");
}

fn read_array(reader: &mut Box<dyn io::BufRead>) -> Vec<Integer> {
    let line = read_line(reader);
    return line
        .trim()
        .split(' ')
        .flat_map(str::parse::<Integer>)
        .collect::<Vec<_>>();
}

fn std_reader() -> Box<dyn io::BufRead> {
    Box::new(io::BufReader::new(io::stdin()))
}

fn read_line(input_reader: &mut Box<dyn io::BufRead>) -> String {
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
            .filter(|line| !line.trim().is_empty())
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
