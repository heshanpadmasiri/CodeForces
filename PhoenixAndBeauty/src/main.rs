use std::{io, collections::HashSet};

type Reader = Box<dyn io::BufRead>;
type Int = usize;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    let t = read_int(reader_ref);
    for _ in 0..t {
        println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
    }
}

#[derive(Clone, Debug)]
struct TestCase {
    n: Int,
    k: Int,
    arr: Vec<Int>
}

struct Answer {
    beautiful_arr: Option<Vec<Int>>
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let data = read_array(reader);
        let n = data[0];
        let k = data[1];
        let arr = read_array(reader);
        Self { n, k, arr }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        match self.beautiful_arr {
            Some(ref arr) => {
                arr.len().to_string() +
                "\n" +
                &arr.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ")}
            None => "-1".to_string()
        }
    }
}

fn solve(case: TestCase) -> Answer {
    let k = case.k;
    let n = case.n;
    let arr = case.arr;
    let mut distinct_arr = Vec::with_capacity(k);
    let mut seen = HashSet::new();
    for each in arr {
        if seen.contains(&each) {
            continue;
        }
        distinct_arr.push(each);
        seen.insert(each);
    }
    if distinct_arr.len() > k {
        return Answer { beautiful_arr: None };
    }
    while distinct_arr.len() < k {
        distinct_arr.push(1);
    }
    let mut beautiful_arr = Vec::with_capacity(n*k);
    while beautiful_arr.len() < n*k {
        beautiful_arr.extend(distinct_arr.iter().cloned());
    }
    return Answer { beautiful_arr: Some(beautiful_arr) };
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

    use crate::{solve, TestCase, read_int, Answer};

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
            let t = read_int(reader_ref);
            let actual = (0..t).map(|_| {
                let case = TestCase::from_input(reader_ref);
                let ans = solve(case.clone());
                validate(ans, &case).to_string()
            }).collect::<Vec<String>>().join("\n") + "\n";
            assert_eq!(actual, test_case_output(&each));
        }
    }

    fn validate(ans: Answer, test_case: &TestCase) -> Answer {
        if let Some(ref arr) = ans.beautiful_arr {
            let k = test_case.k; 
            let sub_sum = (0..k).map(|i| arr[i]).sum::<usize>();
            for i in 1..(test_case.n - k + 1) {
                let sum = (0..k).map(|j| arr[i + j]).sum::<usize>();
                assert_eq!(sum, sub_sum);
            }
        }
        ans
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
            .map(|line| line.expect("failed to read line") + "\n")
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
