use std::io;

fn main() {
    let mut reader = std_reader();
    let reader_ref = &mut reader;
    let n = read_int(reader_ref);
    let nums = read_array(reader_ref);
    let sum: i64 = nums.iter().sum();

    println!("{}", sum);
}

fn read_int(reader: &mut Box<dyn io::BufRead>) -> i64 {
    let line = read_line(reader);
    return line.trim().parse().expect("expect an integer");
}

fn read_array(reader: &mut Box<dyn io::BufRead>) -> Vec<i64> {
    let line = read_line(reader);
    return line.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();
}

fn std_reader() -> Box<dyn io::BufRead> {
    Box::new(io::BufReader::new(io::stdin()))
}

fn read_line(input_reader: &mut Box<dyn io::BufRead>) -> String {
    let mut line = String::new();
    input_reader.read_line(&mut line).expect("expect input line");
    line
}
