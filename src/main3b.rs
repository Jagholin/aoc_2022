use std::fs::read_to_string;
use std::collections::HashSet;

fn get_set(line: &str) -> HashSet<char> {
    let line = line.trim();
    let mut result = HashSet::with_capacity(line.len());
    for c in line.chars() {
        result.insert(c);
    }
    result
}

fn main() {
    let input = read_to_string("input3a.txt").unwrap();
    let mut score_total = 0;
    let mut buf = [0u8; 1];
    'a'.encode_utf8(&mut buf);
    let a_prior = buf[0];
    'A'.encode_utf8(&mut buf);
    let capa_prior = buf[0];
    let mut priority_sum: u32 = 0;
    let mut lines = input.lines();
    loop {
        let next_line = lines.next();
        let next_line = match next_line {
            Some(l) => l,
            None => break
        };
        let line_2 = lines.next().unwrap();
        let line_3 = lines.next().unwrap();

        let next_line = get_set(next_line);
        let line_2 = get_set(line_2);
        let line_3 = get_set(line_3);

        let intersect: HashSet<_> = next_line.intersection(&line_2).copied().collect();
        let intersect: Vec<_> = intersect.intersection(&line_3).collect();
        if intersect.len() != 1 {
            panic!("intersection is not length 1, {:?}", intersect);
        };

        let intersect = **intersect.get(0).unwrap();
        intersect.encode_utf8(&mut buf);
        let priority = if ( intersect.is_ascii_lowercase()) { 
            buf[0] - a_prior + 1
        } else {
            buf[0] - capa_prior + 27
        };
        priority_sum += priority as u32;
    }
    println!("{priority_sum}")
}