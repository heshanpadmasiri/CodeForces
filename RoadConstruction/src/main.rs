use std::io;

type Reader = Box<dyn io::BufRead>;
type Int = usize;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    for line in solve(reader_ref) {
        println!("{}", line);
    }
}

fn solve(input: &mut Reader) -> Vec<String> {
    let mut solution = solve_inner(input);
    let mut ans = Vec::with_capacity(solution.len() + 1);
    ans.push(format!("{}", solution.len()));
    ans.append(&mut solution);
    ans
}

fn solve_inner(input: &mut Reader) -> Vec<String> {
    let data = read_array(input);
    let n = data[0];
    let m = data[1];
    let mut is_leaf = vec![false; n];
    for _ in 0..m {
        let data = read_array(input);
        let u = data[0] - 1;
        let v = data[1] - 1;
        is_leaf[u] = true;
        is_leaf[v] = true;
    }
    let center = center(n, is_leaf).unwrap();
    (0..n)
        .filter(|each| each != &center)
        .map(|each| [each, center])
        .map(|each| format!("{} {}", each[0] + 1, each[1] + 1))
        .collect()
}

fn center(n: Int, is_leaf: Vec<bool>) -> Option<Int> {
    let mut center = None;
    for i in 0..n {
        if !is_leaf[i] {
            center = Some(i);
            break;
        }
    }

    center
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
