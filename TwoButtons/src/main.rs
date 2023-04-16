use std::{collections::HashMap, io};

type Reader = Box<dyn io::BufRead>;
type Int = usize;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
}

struct TestCase {
    n: Int,
    m: Int
}

struct Answer {
    moves: Int,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let data = read_array(reader);
        let n = data[0];
        let m = data[1];
        Self { n, m }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        self.moves.to_string()
    }
}

struct Context {
    current_best: Option<Int>,
    explored_values: HashMap<Int, Int>,
}

fn solve(case: TestCase) -> Answer {
    let mut context = Context { current_best: None, explored_values: HashMap::<Int,Int>::new() };
    let target = case.m;
    let current_value = case.n;
    solve_inner(current_value, target, 0, &mut context);
    return Answer { moves: context.current_best.unwrap() };
}

fn solve_inner(current_value: Int, target: Int, moves: Int, context: &mut Context) {
    if context.current_best.is_some() && moves >= context.current_best.unwrap() {
        // backtrack
        return;
    }
    if current_value == target {
        context.current_best = Some(moves);
        return;
    }
    if let Some(best_sofar) =  context.explored_values.get(&current_value) {
        if best_sofar < &moves {
            return;
        }
    }
    context.explored_values.insert(current_value, moves);
    if current_value > 0 {
        solve_inner(current_value - 1, target, moves + 1, context);
    }
    if current_value < target {
        solve_inner(current_value * 2, target, moves + 1, context);
    }
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
