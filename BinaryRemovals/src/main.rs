use std::io;
use std::collections::HashMap;

fn main() {
    let mut t = String::new();
    io::stdin().read_line(&mut t).expect("Failed to read t");
    let t: i64 = t.trim().parse().expect("Expect t to be integer");
    
    for _ in 0..t {
        if solve() {
            println!("YES");
        }
        else {
            println!("NO");
        }
    }
}

fn solve() -> bool {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    let chars = line.trim().chars().collect::<Vec<char>>();
    let mut dp = HashMap::new();
    return solve_inner(&chars, 0, &init_skip_indicies(), &mut dp);
}

fn solve_inner(vals: &Vec<char>, index: usize, current: &SkipIndicies, dp: &mut HashMap<u128, bool>) -> bool {
    // println!("index : {index} current: {:?}", current.indicies);
    if is_sorted(vals, current, dp) {
        // println!("\tsorted");
        return true;
    }
    if index >= vals.len() {
        // println!("\tend");
        return false;
    }
    let is_skippable;
    if index == 0 || current.size == 0 {
        is_skippable = true;
    }
    else {
        is_skippable = index > current.indicies[current.size - 1] + 1;
    }
    // println!("\tskippable: {is_skippable}");
    return solve_inner(vals, index + 1, current, dp) || (is_skippable && solve_inner(vals, index + 1, &add_skip_index(current, index), dp));
}

fn is_sorted(vals: &Vec<char>, skip_indices: &SkipIndicies, dp: &mut HashMap<u128, bool>) -> bool {
    let memo = dp.get(&skip_indices.hash);
    if memo.is_some() {
        return *memo.unwrap();
    }
    let new_vals = filtere_vals(vals, skip_indices);
    for i in 1 .. new_vals.len() {
        if new_vals[i-1] > new_vals[i] {
            dp.insert(skip_indices.hash, false);
            return false;
        }
    }
    dp.insert(skip_indices.hash, true);
    return true;
}

struct SkipIndicies {
    indicies: Vec<usize>,
    size: usize,
    hash: u128
}

fn init_skip_indicies() -> SkipIndicies {
    return SkipIndicies { indicies: vec![], size: 0, hash: 0 };
}

fn add_skip_index(current: &SkipIndicies, new_index: usize) -> SkipIndicies {
    let mut indicies = current.indicies.to_vec();
    let hash = current.hash | (1 << new_index);
    indicies.push(new_index);
    return SkipIndicies { indicies, size: current.size + 1, hash };
}

fn filtere_vals(vals: &Vec<char>, skip_indices: &SkipIndicies) -> Vec<char> {
    let mut new_vals = vec![];
    new_vals.reserve(vals.len() - skip_indices.indicies.len());
    let mut skip_index = 0;
    for i in 0 .. vals.len() {
        if skip_index >= skip_indices.indicies.len() {
            new_vals.push(vals[i]);
        }
        else if skip_indices.indicies[skip_index] == i {
            skip_index += 1;
        }
        else {
            new_vals.push(vals[i]);
        }
    }
    return new_vals;
}
