use regex::Regex;
use std::fs::read_to_string;

fn half_overlap(a1: i32, a2: i32, b1: i32, _b2: i32) -> bool {
    a1 <= b1 && a2 >= b1
}

fn main() {
    let input = read_to_string("input4a.txt").unwrap();
    let line_rx = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    let mut containment_counter = 0;
    for mut line in input.lines() {
        line = line.trim();
        let caps = line_rx.captures(line).unwrap();
        let caps: Vec<_> = caps
            .iter()
            .skip(1)
            .map(|cap| cap.unwrap().as_str().parse::<i32>().unwrap())
            .collect();
        if half_overlap(caps[0], caps[1], caps[2], caps[3])
            || half_overlap(caps[2], caps[3], caps[0], caps[1])
        {
            containment_counter += 1;
        }
    }
    println!("{}", containment_counter);
}
