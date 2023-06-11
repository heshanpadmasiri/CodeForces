use std::io;

type Reader = Box<dyn io::BufRead>;
type Int = usize;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    println!("{}", solve(TestCase::from_input(reader_ref)).to_string());
}

struct TestCase {
    n: usize,
    m: usize,
    employee_languages: Vec<EmployeeLanguages>,
}

type EmployeeLanguages = Vec<usize>;

struct Answer {
    cost: Int,
}

impl TestCase {
    fn from_input(reader: &mut Reader) -> Self {
        let data = read_array(reader);
        let n = data[0];
        let m = data[1];
        let mut employee_languages = Vec::with_capacity(n);
        for _ in 0..n {
            employee_languages.push(read_array(reader)[1..].to_vec());
        }
        Self {
            n,
            m,
            employee_languages,
        }
    }
}

impl ToString for Answer {
    fn to_string(&self) -> String {
        self.cost.to_string()
    }
}

fn solve(case: TestCase) -> Answer {
    let mut cx = parse_test_case(case);
    // cx.pp();
    let mut n_subcomponents = 0;
    let mut stack: Vec<usize> = Vec::with_capacity(cx.employees.len());
    let mut index = 0;
    loop {
        let employee = cx.get_next_employee(index);
        if employee.is_none() {
            break;
        }
        index = employee.unwrap().id;
        n_subcomponents += 1;
        stack.push(employee.unwrap().id);
        while stack.len() > 0 {
            let employee_id = stack.pop().unwrap();
            let current_employee = cx.employee(employee_id);
            if current_employee.color.is_some() {
                continue;
            }
            current_employee.color = Some(n_subcomponents);
            for each in cx.communicable_new_employees(employee_id) {
                assert!(each.color.is_none());
                stack.push(each.id);
            }
        }
    }
    let noob_count = cx.brand_new_employee_count();
    let cost_of_connecting_subcomponents = if n_subcomponents > 1 {
        n_subcomponents - 1
    } else {
        0
    };
    let cost = cost_of_connecting_subcomponents + noob_count;
    // println!(
    //     "n_subcomponents: {}, brand_new_employee_count: {}, cost: {}",
    //     n_subcomponents,
    //     cx.brand_new_employee_count(),
    //     cost
    // );
    return Answer { cost };
}

type LanguageEmployeeMap = Vec<Vec<usize>>;

struct Context {
    language_employee_map: LanguageEmployeeMap,
    employees: Vec<EmployeeData>,
}

impl Context {
    fn new(language_employee_map: LanguageEmployeeMap, employees: Vec<EmployeeData>) -> Self {
        Self {
            language_employee_map,
            employees,
        }
    }

    fn pp(&self) {
        println!("language_employee_map: {:?}", self.language_employee_map);
        println!("employees: {:?}", self.employees);
    }

    fn get_next_employee(&self, start_index: usize) -> Option<&EmployeeData> {
        self.employees[start_index..]
            .iter()
            .find(|&employee| employee.color.is_none() && employee.languages.len() > 0)
    }

    fn brand_new_employee_count(&self) -> usize {
        self.employees
            .iter()
            .filter(|&employee| employee.languages.len() == 0)
            .count()
    }

    fn employee(&mut self, id: usize) -> &mut EmployeeData {
        &mut self.employees[id]
    }

    fn employees_talking(&self, language: usize) -> &[usize] {
        &self.language_employee_map[language - 1]
    }

    fn communicable_new_employees(&self, employee_id: usize) -> Vec<&EmployeeData> {
        let mut communicable_employees = Vec::new();
        let employee = &self.employees[employee_id];
        for language in &employee.languages {
            communicable_employees.extend(
                self.employees_talking(*language)
                    .iter()
                    .filter(|&other_id| *other_id != employee.id)
                    .map(|&other_id| &self.employees[other_id])
                    .filter(|&other| other.id != employee.id && other.color.is_none())
                    .collect::<Vec<_>>(),
            );
        }
        communicable_employees
    }
}

#[derive(Debug)]
struct EmployeeData {
    id: usize,
    color: Option<usize>,
    languages: EmployeeLanguages,
}

impl EmployeeData {
    fn new(id: usize, languages: EmployeeLanguages) -> Self {
        Self {
            id,
            color: None,
            languages,
        }
    }
}

fn parse_test_case(case: TestCase) -> Context {
    let mut language_employee_map: LanguageEmployeeMap = vec![Vec::new(); case.m];
    let mut employees = Vec::with_capacity(case.n);
    for (employee_id, languages) in case.employee_languages.into_iter().enumerate() {
        let employee_data = EmployeeData::new(employee_id, languages);
        for language in &employee_data.languages {
            language_employee_map[*language - 1].push(employee_id);
        }
        employees.push(employee_data);
    }
    return Context::new(language_employee_map, employees);
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
