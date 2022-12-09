use std::collections::HashSet;
use std::fs::read_to_string;

type Location = (isize, isize);

fn distance2(a: &Location, b: &Location) -> isize {
    (a.0 - b.0).pow(2) + (a.1 - b.1).pow(2)
}

fn move_head(dx: isize, dy: isize, head: Location) -> Location {
    let res = (head.0 + dx, head.1 + dy);
    // print!("Head moves to {} {}, ", res.0, res.1);
    res
}

fn move_tail(head: Location, tail: Location) -> Location {
    // move the tail accordingly
    let mut res = tail.clone();
    let dist = distance2(&head, &tail);
    if dist > 2 {
        // distance is too large, move the tail
        let mut candidates: Vec<_> = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .into_iter()
        .map(|(tdx, tdy)| (tail.0 + tdx, tail.1 + tdy))
        .collect();

        candidates.sort_by(|a, b| distance2(&head, a).cmp(&distance2(&head, b)));
        res = candidates.first().unwrap().to_owned();
        // println!("tail moves to {} {}", res.0, res.1);
    }
    res
}

fn main() {
    let input = read_to_string("input9.txt").unwrap();
    let mut head: Location = (0, 0);
    // let mut tail: Location = (0, 0);
    let mut tails: Vec<_> = (0..9).into_iter().map(|_| (0, 0)).collect();
    let mut tail_positions = HashSet::new();

    for line in input.lines() {
        let mut line_iter = line.split_whitespace();
        let direction = line_iter.next().unwrap();
        let distance = line_iter.next().unwrap();
        let distance = distance.parse::<usize>().unwrap();

        for i in 0..distance {
            head = match direction {
                "R" => move_head(1, 0, head),
                "L" => move_head(-1, 0, head),
                "U" => move_head(0, 1, head),
                "D" => move_head(0, -1, head),
                _ => panic!("unknown direction {direction} in line {line}"),
            };
            let mut prev_head = head;
            for j in 0..tails.len() {
                tails[j] = move_tail(prev_head, tails[j]);
                prev_head = tails[j];
            }
            tail_positions.insert(tails.last().unwrap().clone());
        }
    }

    println!("{}", tail_positions.len());
}
