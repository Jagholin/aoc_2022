use regex::Regex;
use std::fs::read_to_string;

enum Op {
    Add(Option<i32>),
    Mul(Option<i32>),
}

struct Monkey {
    items: Vec<i64>,
    op: Op,
    divisible_by_test: i32,
    if_true_throw_to: usize,
    if_false_throw_to: usize,
    items_inspected: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("input11.txt").unwrap();

    let mut lines_iter = input.lines();
    let header_regex = Regex::new(r"Monkey ((\d)+):")?;
    let items_regex = Regex::new(r"Starting items: (((\d)+,?\s?)+)")?;
    let op_regex = Regex::new(r"Operation: new = old (\+|\*) (old|\d+)")?;
    let test_regex = Regex::new(r"Test: divisible by (\d+)")?;
    let if_regex_true = Regex::new(r"If true: throw to monkey (\d+)")?;
    let if_regex_false = Regex::new(r"If false: throw to monkey (\d+)")?;

    let mut monkeys = vec![];

    let modulus = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19 * 23;

    loop {
        let line = lines_iter.next();
        let line = match line {
            Some(l) => l,
            None => break,
        };
        if !header_regex.is_match(line) {
            panic!("header didnt match: {line}");
        }
        let line = lines_iter.next().unwrap();
        let caps = items_regex.captures(line).unwrap();
        let items_vec: Vec<_> = caps
            .get(1)
            .unwrap()
            .as_str()
            .split(", ")
            .map(|i| i.parse::<i64>().unwrap())
            .collect();
        let line = lines_iter.next().unwrap();
        let caps = op_regex.captures(line).unwrap();
        let op = match (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()) {
            ("+", "old") => Op::Add(None),
            ("*", "old") => Op::Mul(None),
            ("+", num) => Op::Add(Some(num.parse::<i32>().unwrap())),
            ("*", num) => Op::Mul(Some(num.parse().unwrap())),
            _ => panic!("cant match operation: {line}"),
        };
        let line = lines_iter.next().unwrap();
        let caps = test_regex.captures(line).unwrap();
        let test_div = caps.get(1).unwrap().as_str().parse::<i32>()?;
        let line = lines_iter.next().unwrap();
        let caps = if_regex_true.captures(line).unwrap();
        let throw_on_true = caps.get(1).unwrap().as_str().parse::<usize>()?;
        let line = lines_iter.next().unwrap();
        let caps = if_regex_false.captures(line).unwrap();
        let throw_on_false = caps.get(1).unwrap().as_str().parse::<usize>()?;

        monkeys.push(Monkey {
            items: items_vec,
            op,
            divisible_by_test: test_div,
            if_true_throw_to: throw_on_true,
            if_false_throw_to: throw_on_false,
            items_inspected: 0,
        });
        // skip an empty line
        lines_iter.next();
    }

    for _step in 0..10000 {
        // let mut new_monkey_vec = vec![];

        for i in 0..monkeys.len() {
            let mon = std::mem::take(&mut monkeys[i].items);
            monkeys[i].items_inspected += mon.len();

            for mut item in mon {
                item = match monkeys[i].op {
                    Op::Add(Some(num)) => (item + num as i64) % modulus,
                    Op::Mul(Some(num)) => (item * num as i64) % modulus,
                    Op::Add(_) => (item + item) % modulus,
                    Op::Mul(_) => (item * item) % modulus,
                };
                // item = item / 3;
                if item % monkeys[i].divisible_by_test as i64 == 0 {
                    let target = monkeys[i].if_true_throw_to;
                    monkeys[target].items.push(item);
                } else {
                    let target = monkeys[i].if_false_throw_to;
                    monkeys[target].items.push(item);
                }
            }
        }
    }

    monkeys.sort_unstable_by(|m1, m2| m2.items_inspected.cmp(&m1.items_inspected));
    let monkey_business = monkeys[0].items_inspected * monkeys[1].items_inspected;
    println!("{}", monkey_business);

    Ok(())
}
