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

fn solve(reader: &mut Reader) -> Vec<String> {
    let Data { n, k, x } = parse_data(reader);
    let mut levels = read_array(reader);
    levels.sort_unstable();
    let mut diff_arr: Vec<usize> = (1 .. n).filter_map(|i| {
        let diff = levels[i] - levels[i - 1];
        if diff > x {
            Some(diff)
        }
        else {
            None
        }
    }).collect();
    diff_arr.sort_unstable();
    let mut group_count = 1;
    let mut remaining_patches = k;
    for each in diff_arr {
        let required = (each -1 ) / x;
        if remaining_patches >= required {
            remaining_patches -= required;
        }
        else {
            group_count += 1;
        }
    }
    vec![group_count.to_string()]
}


struct Data {
    n: Int,
    k: Int,
    x: Int,
}

fn parse_data(reader: &mut Reader) -> Data {
    let data = read_array(reader);
    Data {
        n: data[0],
        k: data[1],
        x: data[2],
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
