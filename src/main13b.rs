use std::{error::Error, fs::read_to_string, num::ParseIntError, ops::Range, str::FromStr};
use thiserror::Error;

#[derive(Debug, Error)]
enum ParseError {
    #[error("Unexpected symbol {}, expected {}", .0, .1)]
    UnexpectedSymbol(String, String),
    #[error(transparent)]
    Other(#[from] ParseIntError),
}

struct SplitWhileMaskingBrackets<'a> {
    input: &'a str,
    tail: Range<usize>,
}

impl<'a> SplitWhileMaskingBrackets<'a> {
    fn new(from: &'a str) -> Self {
        Self {
            input: from,
            tail: 0..from.len(),
        }
    }
}

impl<'a> Iterator for SplitWhileMaskingBrackets<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tail.end == self.tail.start {
            // 0 len tail
            return None;
        }
        let mut brackets_depth = 0;
        let mut result = None;
        for c_pos in self.tail.clone() {
            match &self.input[c_pos..c_pos + 1] {
                "[" => {
                    brackets_depth += 1;
                    continue;
                }
                "]" => {
                    brackets_depth -= 1;
                    continue;
                }
                _ => (),
            }
            if brackets_depth > 0 {
                continue;
            }
            if self.input[c_pos..].starts_with(",") {
                result = Some(&self.input[self.tail.start..c_pos]);
                self.tail.start = c_pos + 1;
                break;
            }
        }
        if result.is_none() {
            // no commas remaining, return the entire tail
            result = Some(&self.input[self.tail.clone()]);
            self.tail.start = self.tail.end;
        }
        result
    }
}

#[derive(Debug)]
enum BracketListItem {
    Number(i32),
    List(Vec<BracketListItem>),
}

impl FromStr for BracketListItem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("from str called: {s}");
        let first_char = s.get(0..1).ok_or(ParseError::UnexpectedSymbol(
            "eof".to_string(),
            "[ or number".to_string(),
        ))?;
        match first_char {
            "[" => {
                // the last character should be "]"
                if !s.ends_with("]") {
                    return Err(ParseError::UnexpectedSymbol(
                        s[s.len() - 2..].to_string(),
                        "]".to_string(),
                    ));
                }
                let insides = &s[1..s.len() - 1];
                let vec_res: Result<Vec<_>, _> = SplitWhileMaskingBrackets::new(insides)
                    .map(|v| v.parse::<BracketListItem>())
                    .collect();
                // let vec_res: Result<Vec<_>, _> = insides.split(",").map(|v| v.parse::<BracketListItem>()).collect();
                let vec_res = vec_res?;
                Ok(BracketListItem::List(vec_res))
            }
            _ => {
                // expected a number
                let num = s.parse::<i32>()?;
                Ok(BracketListItem::Number(num))
            }
        }
    }
}

impl PartialEq for BracketListItem {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(l0), Self::Number(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for BracketListItem {}

impl PartialOrd for BracketListItem {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Number(n), Self::Number(m)) => n.partial_cmp(m),
            (Self::List(n), Self::List(m)) => n.iter().partial_cmp(m.iter()),
            (Self::Number(n), Self::List(m)) => {
                Self::List(vec![Self::Number(*n)]).partial_cmp(other)
            }
            (Self::List(n), Self::Number(m)) => {
                self.partial_cmp(&Self::List(vec![Self::Number(*m)]))
            }
        }
    }
}

impl Ord for BracketListItem {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input13.txt").unwrap();

    let mut packets = vec![];
    for (i, pairs) in input.split("\n\n").enumerate() {
        // println!("pair is {pairs}");
        let (line1, line2) = {
            let mut iter = pairs.lines();
            let la = iter.next().unwrap();
            let lb = iter.next().unwrap();
            (la, lb)
        };
        let br1 = line1.parse::<BracketListItem>()?;
        let br2 = line2.parse::<BracketListItem>()?;
        packets.push(br1);
        packets.push(br2);
    }

    packets.push(BracketListItem::List(vec![BracketListItem::List(vec![
        BracketListItem::Number(2),
    ])]));
    packets.push(BracketListItem::List(vec![BracketListItem::List(vec![
        BracketListItem::Number(6),
    ])]));

    packets.sort_unstable();
    // println!("packets is {:#?}", packets);
    let first_packet = packets
        .binary_search(&BracketListItem::List(vec![BracketListItem::List(vec![
            BracketListItem::Number(2),
        ])]))
        .unwrap();
    let second_packet = packets
        .binary_search(&BracketListItem::List(vec![BracketListItem::List(vec![
            BracketListItem::Number(6),
        ])]))
        .unwrap();

    println!(
        "The decoder key is {}",
        (first_packet + 1) * (second_packet + 1)
    );
    Ok(())
}
