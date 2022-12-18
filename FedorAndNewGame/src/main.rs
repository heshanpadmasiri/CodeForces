use std::io;

fn main() {
    let data = read_array();
    // let n = data[0];
    let m = data[1];
    let k = data[2];
    let players = (1..=(m+1)).map(|_| read_int()).collect::<Vec<_>>();
    let fedore = players[m];
    let friend_count: usize = (0..m).map(|i| diff_count(players[i], fedore)).filter(|c| c <= &k).count();
    println!("{}", friend_count);
}

fn diff_count(a: usize, b: usize) -> usize {
    let diff = a ^ b;
    let mut diff_count = 0;
    for i in 0..21 {
        let probe = 1 << i;
        if (diff & probe) != 0 {
            diff_count += 1;
        }
    }
    // println!("\t {a} {b} {diff} {diff_count}");
    return diff_count;
}

fn read_int() -> usize {
    let line = read_line();
    return line.trim().parse().expect("expect an integer");
}

fn read_array() -> Vec<usize> {
    let line = read_line();
    return line.trim().split(' ').flat_map(str::parse::<usize>).collect::<Vec<_>>();
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    return line;
}
