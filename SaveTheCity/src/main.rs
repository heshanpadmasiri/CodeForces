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
    let n = read_int(input);
    (0..n).map(|_| min_cost(input).to_string()).collect()
}

enum State {
    Empty,
    Bomb,
    PlacedBomb(Int)
}

fn min_cost(input: &mut Reader) -> Int {
    let data = read_array(input);
    let cost_to_blow_up = data[0];
    let cost_to_place_new = data[1];
    let bombs = read_bit_array(input);
    let mut cost = 0;
    let mut i = 0;
    let mut state = State::Empty;
    while i < bombs.len() {
        match state {
            State::Empty => {
                if bombs[i] {
                    state = State::Bomb;
                    cost += cost_to_blow_up;
                }
            }
            State::Bomb => {
                if !bombs[i] {
                   state = State::PlacedBomb(cost_to_place_new);
                }
            }
            State::PlacedBomb(current_cost) => {
                if current_cost > cost_to_blow_up {
                    state = State::Empty;
                    continue;
                }
                if bombs[i] {
                    state = State::Bomb;
                    cost += current_cost;
                }
                else {
                    state = State::PlacedBomb(current_cost + cost_to_place_new);
                }
            }
        }
        i += 1;
    }
    cost
}

fn read_int(reader: &mut Reader) -> Int {
    let line = read_line(reader);
    return line.trim().parse().expect("expect an integer");
}

fn read_array(reader: &mut Reader) -> Vec<Int> {
    let line = read_line(reader);
    return line.trim().split(' ').flat_map(str::parse::<Int>).collect::<Vec<_>>();
}

fn read_bit_array(reader: &mut Reader) -> Vec<bool> {
    read_line(reader).trim().chars().map(|c| c == '1').collect()
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
