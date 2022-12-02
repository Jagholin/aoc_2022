use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    let input = read_to_string("input2a.txt").unwrap();
    let mut score_total = 0;
    for mut line in input.lines() {
        line = line.trim();
        let subs: Vec<_> = line.split_whitespace().collect();
        let sub1 = subs.get(0).unwrap().trim();
        let sub2 = subs.get(1).unwrap().trim();
        match sub1 {
            "A" => {
                // rock
                if sub2 == "X" { 
                    score_total += 4;
                } else if sub2 == "Y" {
                    score_total += 8;
                } else if sub2 == "Z" {
                    score_total += 3;
                }
            },
            "B" => {
                // paper
                if sub2 == "X" { 
                    score_total += 1;
                } else if sub2 == "Y" {
                    score_total += 5;
                } else if sub2 == "Z" {
                    score_total += 9;
                }
            },
            "C" => {
                // scissors
                if sub2 == "X" { 
                    score_total += 7;
                } else if sub2 == "Y" {
                    score_total += 2;
                } else if sub2 == "Z" {
                    score_total += 6;
                }
            },
            _ => panic!("unknown select: {sub1}"),
        }
    }
    println!("{score_total}")
}