use std::io;
use std::slice;

type Reader = Box<dyn io::BufRead>;
type Int = usize;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    let n = read_int(reader_ref);
    for _ in 0..n {
        println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
    }
}

#[derive(Debug)]
struct TestCase {
    n: Int,
    university_students: Vec<Vec<Int>>,
}

struct Answer {
    team_strengths: Vec<Int>,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let n = read_int(reader);
        let student_university = read_array(reader);
        let strengths = read_array(reader);
        let mut university_students = vec![Vec::new(); n as usize];
        for (idx, strength) in strengths.iter().enumerate() {
            university_students[student_university[idx] as usize - 1].push(*strength);
        }
        Self {
            n,
            university_students,
        }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        self.team_strengths
            .iter()
            .map(|strength| strength.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn solve(case: TestCase) -> Answer {
    let strengths = sort_and_cleanup(case.university_students);
    let team_strengths = (1..=case.n)
        .map(|team_size| regional_strength(&strengths, team_size as usize))
        .collect();
    Answer { team_strengths }
}

fn regional_strength(strengths: &Vec<Vec<Int>>, team_size: usize) -> Int {
    strengths
        .iter()
        .take_while(|university| university.len() >= team_size)
        .map(|university| university_strength(university, team_size))
        .sum()
}

fn university_strength(prefix_sum: &Vec<Int>, team_size: usize) -> Int {
    if prefix_sum.len() < team_size {
        return 0;
    }
    let n_teams = (prefix_sum.len() / team_size) * team_size;
    return prefix_sum[n_teams - 1];
}

fn prefix_sum(vals: &Vec<Int>) -> Vec<Int> {
    let mut prefix_sum = Vec::with_capacity(vals.len());
    let mut sum = 0;
    for val in vals {
        sum += val;
        prefix_sum.push(sum);
    }
    prefix_sum
}

fn sort_and_cleanup(university_students: Vec<Vec<Int>>) -> Vec<Vec<Int>> {
    let mut data: Vec<Vec<Int>> = university_students
        .iter()
        .filter_map(|strengths| match strengths.len() {
            0 => None,
            _ => {
                let mut strengths = strengths.clone();
                strengths.sort_unstable_by(|a, b| b.cmp(a));
                Some(prefix_sum(&strengths))
            }
        })
        .collect();
    data.sort_unstable_by(|a, b| b.len().cmp(&a.len()));
    data
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
