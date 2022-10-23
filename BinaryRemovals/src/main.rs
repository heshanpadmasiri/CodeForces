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
    // TODO: special case less than 2
    if chars.len() <= 2 {
        return true;
    }
    let mut dp = HashMap::new();
    return solve_inner(&chars, 0, None, None, &mut dp);
}

fn solve_inner(chars: &Vec<char>, index: usize, last_val: Option<char>, last_skip_index: Option<usize>, dp: &mut HashMap<DpKey, bool>) -> bool {
    // println!("index : {}, last_val: {:?}, last_skip_index: {:?}", index, last_val, last_skip_index);
    if index >= chars.len() {
        return true;
    }
    let key = DpKey { index, last_val, last_skip_index };
    let memo = dp.get(&key);
    if memo.is_some() {
        return *memo.unwrap();
    }
    let mut must_skip = false;
    if last_val.is_some() {
        let top = last_val.unwrap();
        if top > chars[index] {
            must_skip = true;
        }
    }
    // lhs is sorted
    if !must_skip {
        let inclusive = solve_inner(chars, index + 1, Some(chars[index]), last_skip_index, dp);
        if inclusive {
            return true;
        }
    }
    let skippable;
    if last_skip_index.is_some() {
        skippable = last_skip_index.unwrap() + 1 < index; 
    }
    else {
        skippable = true;
    }
    if skippable {
        return solve_inner(chars, index + 1, last_val, Some(index), dp); 
    }
    dp.insert(key, false);
    return false;
}
#[derive(Eq, Hash, PartialEq)] 
struct DpKey {
    index: usize,
    last_val: Option<char>,
    last_skip_index: Option<usize>
}
