use std::io;
use std::env;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let t = read_int();
    for _ in 0..t {
        println!("{}", solve());
    }
}

#[derive(Copy, Clone, Debug)]
struct OptionResult {
    items: usize,
    remainder: usize,
    cost: usize
}

const INF: OptionResult = OptionResult { items: usize::MIN, remainder: usize::MIN, cost: usize::MAX };

#[derive(Debug)]
struct DpData {
    skip: OptionResult,
    buy_without_offer: OptionResult,
    buy_last_k_with_offer: OptionResult
}

fn solve() -> i64 {
    let data = read_array_as_usize();
    let n = data[0];
    let p = data[1];
    let k = data[2];
    let mut prices = read_array_as_usize();
    prices.sort();
    if n == 0 || prices[0] > p {
        return 0;
    }
    let mut dp = Vec::<DpData>::new();
    dp.reserve(n);
    for i in 0..n {
        let item_price = prices[i];
        if item_price > p {
            break;
        }
        let remainder = p - item_price;
        let skip: OptionResult;
        let buy_without_offer: OptionResult;
        let buy_last_k_with_offer: OptionResult;
        if i == 0 {
            skip = OptionResult { items: 0, remainder: p, cost: 0 }; 
            buy_without_offer = OptionResult { items: 1, remainder, cost: item_price };
            buy_last_k_with_offer = INF;
        } 
        else {
            skip = pick_best_option(&dp[i-1], p).expect("a");
            buy_without_offer = add_items_to_option(&pick_best_option(&dp[i-1], remainder), item_price, 1);
            if i == k-1 {
                buy_last_k_with_offer = OptionResult { items: k, remainder, cost: item_price };
            }
            else if i >= k {
                buy_last_k_with_offer = add_items_to_option(&pick_best_option(&dp[i-k], remainder), item_price, k);
            }
            else {
                buy_last_k_with_offer = INF;
            }
        }
        let tmp = DpData { skip, buy_without_offer, buy_last_k_with_offer };
        // println!("{i} -> {:?}", tmp);
        dp.push(tmp);
    }
    // dp may have less items than n
    return pick_best_option(&dp.pop().expect("empty dp"), p).expect("b").items as i64;
}

fn add_items_to_option(option: &Option<OptionResult>, price: usize, n: usize) -> OptionResult {
    if option.is_none() {
        return INF;
    }
    let option = option.expect("");
    let new_items = option.items + n;
    let remainder = option.remainder - price;
    let cost = option.cost + price;
    OptionResult { items: new_items, remainder, cost }
}

fn pick_best_option(data: &DpData, for_price: usize) -> Option<OptionResult> {
    let mut options = [&data.skip, &data.buy_without_offer, &data.buy_last_k_with_offer];
    // options.sort_by_key(|option| [usize::MAX - option.items, option.cost]);
    options.sort_by(|a, b| if a.items == b.items { a.cost.cmp(&b.cost) } else { b.items.cmp(&a.items) });
    for each in options {
        if each.cost <= for_price {
            return Some(*each);
        }
    }
    None
}

fn read_int() -> i64 {
    let line = read_line();
    return line.trim().parse().expect("expect an integer");
}

fn read_array_as_usize() -> Vec<usize> {
    let line = read_line();
    return line.trim().split(' ').flat_map(str::parse::<usize>).collect::<Vec<_>>();
}

fn read_line() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("expect input line");
    return line;
}
