use std::io;

type Reader = Box<dyn io::BufRead>;
type Int = u64;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    for line in solve(reader_ref) {
        println!("{}", line);
    }
}

fn solve(input: &mut Reader) -> Vec<String> {
    let t = read_int(input);
    (0..t).map(|_| {find_min_increase(input).to_string()}).collect()
}

fn find_min_increase(input: &mut Reader) -> Int {
    // println!("=======");
    let data = read_array(input);
    let k = data[1];
    let vals = read_array(input);
    let predicate = build_predicte(vals, k);
    binary_search(0, 1_000_000_000_000, predicate)
}

fn binary_search(min: Int, max: Int, predicate: Box<dyn Fn(Int) -> bool>) -> Int {
    let mid = (min + max) / 2;
    if mid == min {
        if predicate(min) {
            return min;
        }
        return max;
    }
    if predicate(mid) {
        binary_search(min, mid, predicate)
    }
    else {
        binary_search(mid, max, predicate)
    }
}

fn build_predicte(starting_values: Vec<Int>, k:Int) -> Box<dyn Fn(Int) -> bool> {
    Box::new(
        move |inc| {
            let mut new_values = starting_values.clone();
            new_values[0] += inc;
            // println!("{inc} -> {:?}", new_values);
            is_valid(&new_values, k)
        }
    )
}

fn is_valid(values: &[Int], k: Int) -> bool {
    let target = (k as f64) / 100.0;
    values.iter().skip(1).try_fold(values[0], |d, n| {
        let val = (*n as f64) / (d as f64);
        if val <= target {
            Some(n+d)
        }
        else {
            None
        }
    }).is_some()
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
