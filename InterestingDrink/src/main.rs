use std::io;

fn main() {
    let _num_shops = read_int();
    let mut prices = read_array();
    let num_day = read_int();
    prices.sort_unstable();
    let mut ans = Vec::with_capacity(num_day);
    for _ in 0..num_day {
        let max_coins = read_int();
        ans.push(prices.partition_point(|&x| x <= max_coins));
    }
    for each in ans {
        println!("{each}");
    }
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
