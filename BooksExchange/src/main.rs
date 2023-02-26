use std::{io, collections::HashMap};

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
    (0..n).map(|_| {
        let cycle_lens: Vec<String> = solve_one(input).into_iter().map(|each| {each.to_string()}).collect();
        cycle_lens.join(" ")
    }).collect()
}

fn solve_one(input: &mut Reader) -> Vec<Int> {
    let n = read_int(input);
    let edges = read_array(input);
    let graph = build_graph(edges);
    let mut known_solutions = HashMap::new();
    (1..=n)
        .map(|i| cycle_len(&graph, &mut known_solutions, i))
        .collect()
}

type Graph = HashMap<Int, Int>;

fn build_graph(edges: Vec<Int>) -> Graph {
    let mut graph = HashMap::new();
    for i in 0..edges.len() {
        graph.insert(i+1, edges[i]);
    }
    graph
}

fn cycle_len(graph: &Graph, known_solutions: &mut Graph, start: Int) -> Int {
    if known_solutions.contains_key(&start) {
        return known_solutions[&start];
    }
    let mut set_members = vec![start];
    let mut current = graph[&start];
    set_members.push(current);
    let mut len = 1;
    while current != start {
        current = graph[&current];
        set_members.push(current);
        len += 1;
    }
    for each in set_members {
        known_solutions.insert(each, len);
    }
    len
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
    line.trim().to_string()
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
