use std::io;
use std::cmp;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("failed to read t");
    let t: i64 = t.trim().parse().expect("expect to be an integer");

    for _ in 0..t {
        println!("{}", solve());
    }
}

fn solve() -> i64 {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("failed to read data line");
    let data = data.trim().split(' ').flat_map(str::parse::<i64>).collect::<Vec<_>>();
    let n = data[0] as usize;
    let m = data[1] as usize;
    let x = data[2];
    let y = data[3];
    
    let mut ans = 0;

    for _ in 0..n {
        ans += solve_line(x, y, m);
        // println!("---");
    }

    return ans;
}

fn solve_line(x: i64, y: i64, m: usize) -> i64 {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("failed to read line");
    let chars = line.chars().collect();
    let mut dp = vec![-1; m];
    return solve_line_inner(&chars, &mut dp, 0, x, y, m);
}

fn solve_line_inner(line: &Vec<char>, dp:&mut Vec<i64>, current_index: usize, x: i64, y: i64, m: usize) -> i64 {
    // println!("\t\t{m} {current_index} {current_cost} {:?}", dp);
    if current_index < m && dp[current_index] != -1 {
        // println!("dp hit");
        return dp[current_index];
    }
    let mut index = current_index;
    while index < m && line[index] == '*' {
        // println!("\t\t--skipped{index}, {}", line[index]);
        index += 1;
    }
    if index >= m {
        return 0;
    }
    if index < m-1 && line[index+1] == '.' {
        let single_tile_cost = x + solve_line_inner(line, dp, index + 1, x, y, m);
        let double_tile_cost = y + solve_line_inner(line, dp, index + 2, x, y, m);
        let ans = cmp::min(single_tile_cost, double_tile_cost);
        dp[current_index] = ans;
        return ans;
    }
    let ans = x + solve_line_inner(line, dp, index + 1, x, y, m);
    dp[current_index] = ans;
    return ans;
}
