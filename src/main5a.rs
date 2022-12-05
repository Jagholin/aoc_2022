use regex::Regex;
use std::fs::read_to_string;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let input = read_to_string("input5a.txt").unwrap();
    let line_rx = Regex::new(r"(?:(\d+)\s*)+").unwrap();
    let cargo_rx = Regex::new(r"\[\w\]").unwrap();

    let mut cargo_lines = vec![];
    let mut lines_iter = input.lines().peekable();

    loop {
        let line = *lines_iter.peek().unwrap();
        let mut cargo_line = vec![];
        if line_rx.captures(line).is_some() {
            break;
        }
        lines_iter.next().unwrap();
        let _:Vec<_> = cargo_rx.find_iter(line).map(|m| {
            cargo_line.push((line.get(m.start() + 1..m.start() + 2).unwrap(), m.start() + 1));
            m.start() + 1
        }).collect();
        println!("line end {:?}", cargo_line);
        cargo_lines.push(cargo_line);
    }

    let column_label_rx = Regex::new(r"\d+").unwrap();
    let column_labels = lines_iter.next().unwrap();

    let mut columns = HashMap::new();
    let mut column_offsets = HashMap::new();

    for label in column_label_rx.find_iter(column_labels) {
        let pos = label.start();
        let label = label.as_str().parse::<i32>().unwrap();
        columns.insert(label, vec![]);
        column_offsets.insert(pos, label);
    }

    for cl in cargo_lines.iter().rev() {
        for (c_name, c_offset) in cl.iter() {
            let index = *column_offsets.get(c_offset).unwrap();
            columns.get_mut(&index).unwrap().push(*c_name);
        }
    }

    let command_rx = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for command in lines_iter {
        let cm = command_rx.captures(command);
        let cm = match cm {
            Some(c) => c,
            None => {
                println!("cant match text: {}", command);
                continue;
            },
        };
        let amount = cm.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = cm.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let to = cm.get(3).unwrap().as_str().parse::<i32>().unwrap();

        let mut from_vec = columns.get_mut(&from).unwrap();

        let remainder = from_vec.split_off(from_vec.len() - amount);
        let mut to_vec = columns.get_mut(&to).unwrap();
        for item in remainder.iter().rev() {
            to_vec.push(*item);
        }
    }

    println!("after commands: {:?}", columns);
    for i in 1..=9 {
        print!("{}", columns.get(&i).unwrap().last().unwrap());
    }
}