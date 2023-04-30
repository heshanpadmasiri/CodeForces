use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::io;

type Reader = Box<dyn io::BufRead>;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
}

struct TestCase {
    sum_min: usize,
    sum_max: usize,
    diff_min: usize,
    difficulties: Vec<usize>,
}

struct Answer {
    n_problems: usize,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let line_1 = read_array(reader);
        let sum_min = line_1[1];
        let sum_max = line_1[2];
        let diff_min = line_1[3];
        let mut difficulties = read_array(reader);
        difficulties.sort();
        Self {
            sum_max,
            sum_min,
            diff_min,
            difficulties,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ProblemSet {
    bit_set: usize,
    difficulty: usize,
    starting_index: usize,
    end_index: usize,
    n_problems: usize,
}

impl ProblemSet {
    fn new(starting_index: usize, end_index: usize, difficulty: usize) -> Self {
        let mut bit_set = 0;
        bit_set |= 1 << starting_index;
        bit_set |= 1 << end_index;
        Self {
            bit_set,
            starting_index,
            end_index,
            difficulty,
            n_problems: 2,
        }
    }

    fn add_problem(&self, index: usize, difficulty: usize) -> Self {
        assert!(index >= self.starting_index && index <= self.end_index);
        let mut new_set = self.clone();
        if new_set.bit_set & (1 << index) > 0 {
            return new_set;
        }
        new_set.n_problems += 1;
        new_set.bit_set |= 1 << index;
        new_set.difficulty += difficulty;
        new_set
    }
}

struct Context {
    visited: HashSet<usize>,
}

impl Context {
    fn new() -> Self {
        Self {
            visited: HashSet::new(),
        }
    }
    fn visited(&self, problem_set: &ProblemSet) -> bool {
        self.visited.contains(&problem_set.bit_set)
    }

    fn add_to_visited(&mut self, problem_set: &ProblemSet) {
        self.visited.insert(problem_set.bit_set);
    }
}

impl PartialEq for ProblemSet {
    fn eq(&self, other: &Self) -> bool {
        self.bit_set == other.bit_set
    }
}
impl Eq for ProblemSet {}

impl Hash for ProblemSet {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bit_set.hash(state);
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        self.n_problems.to_string()
    }
}

fn solve(case: TestCase) -> Answer {
    let mut problem_sets = HashSet::new();
    let sum_min = case.sum_min;
    let sum_max = case.sum_max;
    for problem_set in derive_initial_pairs(&case.difficulties, case.diff_min) {
        for n in 2..=case.difficulties.len() {
            let mut cx = Context::new();
            if let Some(valid_problem_sets) = valid_problem_set_with_n_problems(
                &mut cx,
                problem_set,
                sum_min,
                sum_max,
                n,
                &case.difficulties,
            ) {
                problem_sets.extend(valid_problem_sets);
            }
        }
    }
    Answer {
        n_problems: problem_sets.len(),
    }
}

fn problem_set_valid(problem_set: &ProblemSet, sum_min: usize, sum_max: usize) -> bool {
    problem_set.difficulty >= sum_min && problem_set.difficulty <= sum_max
}

fn valid_problem_set_with_n_problems(
    context: &mut Context,
    current_problem: ProblemSet,
    sum_min: usize,
    sum_max: usize,
    n_problems: usize,
    difficulties: &[usize],
) -> Option<HashSet<ProblemSet>> {
    if context.visited(&current_problem) {
        return None;
    } else {
        context.add_to_visited(&current_problem);
    }
    if current_problem.n_problems > n_problems {
        return None;
    }
    if current_problem.n_problems == n_problems {
        if problem_set_valid(&current_problem, sum_min, sum_max) {
            let mut valid_problems = HashSet::new();
            valid_problems.insert(current_problem);
            return Some(valid_problems);
        } else {
            return None;
        }
    }
    let mut valid_problems = HashSet::new();
    for i in current_problem.starting_index..=current_problem.end_index {
        let difficulty = difficulties[i];
        let new_problem_set = current_problem.add_problem(i, difficulty);
        if new_problem_set != current_problem {
            if let Some(problems) = valid_problem_set_with_n_problems(
                context,
                new_problem_set,
                sum_min,
                sum_max,
                n_problems,
                difficulties,
            ) {
                valid_problems.extend(problems);
            }
        }
    }
    if valid_problems.len() > 0 {
        Some(valid_problems)
    } else {
        None
    }
}

// find the hardest and easiest problem in the set
// return problem set with those questions, starting and ending indices for search
fn derive_initial_pairs(difficulties: &[usize], diff_min: usize) -> Vec<ProblemSet> {
    let mut ans = Vec::new();
    for i in 0..difficulties.len() {
        for j in i + 1..difficulties.len() {
            let difficulty_diff = difficulties[j] - difficulties[i];
            if difficulty_diff >= diff_min {
                let problem_set = ProblemSet::new(i, j, difficulties[i] + difficulties[j]);
                ans.push(problem_set);
            }
        }
    }
    ans
}

fn read_array(reader: &mut Reader) -> Vec<usize> {
    let line = read_line(reader);
    return line
        .trim()
        .split(' ')
        .flat_map(str::parse::<usize>)
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
