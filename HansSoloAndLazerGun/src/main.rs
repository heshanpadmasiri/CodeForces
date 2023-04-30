use std::{
    collections::HashSet,
    io,
};

type Reader = Box<dyn io::BufRead>;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
}

struct CoOrdinate {
    x: i64,
    y: i64,
}

struct TestCase {
    starting_co_ordinate: CoOrdinate,
    enemy_co_ordinates: Vec<CoOrdinate>,
}

struct Answer {
    n_shots: usize,
}

#[derive(PartialEq, Eq, Hash, Debug)]
enum Gradient {
    Positive (usize, usize),
    Negative (usize, usize)
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let line_1 = read_array(reader);
        let n = line_1[0];
        let starting_co_ordinate = CoOrdinate {
            x: line_1[1],
            y: line_1[2],
        };
        let enemy_co_ordinates = (0..n).map(|_| TestCase::read_coordinate(reader)).collect();
        Self {
            starting_co_ordinate,
            enemy_co_ordinates,
        }
    }

    fn read_coordinate(reader: &mut Reader) -> CoOrdinate {
        let line = read_array(reader);
        return CoOrdinate {
            x: line[0],
            y: line[1],
        };
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        self.n_shots.to_string()
    }
}

fn solve(case: TestCase) -> Answer {
    let mut shots = HashSet::new();
    for each in case.enemy_co_ordinates {
        let g = gradient(&case.starting_co_ordinate, &each);
        shots.insert(g);
    }
    Answer {
        n_shots: shots.len(),
    }
}

fn gradient(a: &CoOrdinate, b: &CoOrdinate) -> Gradient {
    let start;
    let end;
    if a.y < b.y {
        start = a;
        end = b;
    }
    else {
        start = b;
        end = a;
    }
    let x = end.x - start.x;
    let y = end.y - start.y;
    match (x, y) {
        (0, _) => {
            Gradient::Positive(0, 1)
        },
        (_, 0) => {
            Gradient::Positive(1, 0)
        },
        _ => {
            let x_abs = x.abs() as usize;
            let y_abs = y.abs() as usize;
            let divisor = gcd(x_abs, y_abs);
            if (x > 0 && y > 0) || (x < 0 && y < 0) {
                Gradient::Positive(x_abs / divisor, y_abs / divisor)
            }
            else {
                Gradient::Negative(x_abs / divisor, y_abs / divisor)
            }
        }
    }
}
fn gcd(mut n: usize, mut m: usize) -> usize {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

fn read_array(reader: &mut Reader) -> Vec<i64> {
    let line = read_line(reader);
    return line
        .trim()
        .split(' ')
        .flat_map(str::parse::<i64>)
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
