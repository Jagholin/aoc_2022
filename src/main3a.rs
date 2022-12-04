use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input3a.txt").unwrap();
    let mut score_total = 0;
    let mut buf = [0u8; 1];
    'a'.encode_utf8(&mut buf);
    let a_prior = buf[0];
    'A'.encode_utf8(&mut buf);
    let capa_prior = buf[0];
    let mut priority_sum: u32 = 0;
    for mut line in input.lines() {
        line = line.trim();
        let line_length = line.len();
        // has to be even
        if line_length % 2 == 1 {
            panic!("Line not even: {}", line);
        };
        let mut set_left = HashSet::with_capacity(line_length / 2);
        let mut set_right = HashSet::with_capacity(line_length / 2);

        let slines = line.split_at(line_length / 2);
        for c in slines.0.chars() {
            set_left.insert(c);
        }
        for c in slines.1.chars() {
            set_right.insert(c);
        }
        let intersect: Vec<_> = set_left.intersection(&set_right).collect();
        if intersect.len() != 1 {
            panic!("intersection is not length 1, {:?}", intersect);
        };

        let intersect = **intersect.get(0).unwrap();
        intersect.encode_utf8(&mut buf);
        let priority = if (intersect.is_ascii_lowercase()) {
            buf[0] - a_prior + 1
        } else {
            buf[0] - capa_prior + 27
        };
        priority_sum += priority as u32;
    }
    println!("{priority_sum}")
}
