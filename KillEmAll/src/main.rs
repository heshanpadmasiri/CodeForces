use std::io;

type Reader = Box<dyn io::BufRead>;
type Int = i64;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    let test_cases = read_int(reader_ref);
    (0..test_cases).for_each(|_| {
        println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
    });
}

struct TestCase {
    n: Int,
    r: Int,
    positions: Vec<Int>,
}

struct Answer {
    min_actions: Int,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let data = read_array(reader);
        let n = data[0];
        let r = data[1];
        let positions = read_array(reader);
        Self { n, r, positions }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        self.min_actions.to_string()
    }
}

fn solve(case: TestCase) -> Answer {
    let positions = sorted_unique_positions(&case.positions);
    let mut min_actions = 0;
    let mut edge = 0;
    let mut top_index = positions.len() - 1;
    while positions[top_index] > edge {
        edge += case.r;
        min_actions += 1;
        if top_index == 0 {
            break;
        }
        top_index -= 1;
    }
    Answer { min_actions } 
}

fn sorted_unique_positions(positions: &Vec<Int>) -> Vec<Int> {
    let mut positions = positions.clone();
    positions.sort_unstable();
    positions.dedup();
    positions
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
            let test_cases = read_int(reader_ref);
            let actual:String = (0..test_cases).map(|_| {
                solve(TestCase::from_input(reader_ref)).to_string()
            }).collect::<Vec<_>>().join("\n");
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
            .map(|line| {line.expect("failed to read line")})
            .collect::<Vec<_>>().join("\n")
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
