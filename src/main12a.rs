use std::collections::HashSet;
use std::fs::read_to_string;

#[derive(Clone, Debug)]
struct PathNode {
    pos: (usize, usize),
    coming_from: Option<(usize, usize)>,
    height: u8,
}

impl PartialEq for PathNode {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos
    }
}

impl Eq for PathNode {}

impl std::hash::Hash for PathNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
    }
}

fn main() {
    let input = read_to_string("input12.txt").unwrap();

    let mut height_map = vec![];
    let mut start_pos = (0, 0);
    let mut end_pos = (0, 0);

    let mut buf = [0; 2];
    'a'.encode_utf8(&mut buf);
    let a_encoded = buf[0];

    for (line_num, line) in input.lines().enumerate() {
        let mut height_line = vec![];
        for (char_num, my_char) in line.chars().enumerate() {
            let height = match my_char {
                'S' => {
                    start_pos = (line_num, char_num);
                    0
                }
                'E' => {
                    end_pos = (line_num, char_num);
                    25
                }
                c => {
                    let mut buf = [0; 2];
                    c.encode_utf8(&mut buf);
                    buf[0] - a_encoded
                }
            };
            height_line.push(height);
        }
        height_map.push(height_line);
    }

    let max_x = height_map.len() as i32;
    let max_y = height_map[0].len() as i32;

    let mut visited_nodes = HashSet::new();
    let mut current_nodes = HashSet::from([PathNode {
        pos: start_pos,
        coming_from: None,
        height: 0,
    }]);
    let mut next_nodes = HashSet::new();
    let mut visited_end = false;

    loop {
        for node in current_nodes {
            let candidates: Vec<_> = [(-1, 0), (1, 0), (0, -1), (0, 1)]
                .into_iter()
                .filter_map(|(dx, dy)| {
                    let newx = node.pos.0 as i32 + dx;
                    let newy = node.pos.1 as i32 + dy;
                    if newx >= 0 && newy >= 0 && newx < max_x && newy < max_y {
                        let newx = newx as usize;
                        let newy = newy as usize;
                        let height_diff = (height_map[newx][newy] as i32) - node.height as i32;
                        if height_diff > 1 {
                            None
                        } else {
                            Some(PathNode {
                                pos: (newx, newy),
                                coming_from: Some(node.pos),
                                height: height_map[newx][newy],
                            })
                        }
                    } else {
                        None
                    }
                })
                .filter(|v| !visited_nodes.contains(v))
                .collect();
            for c in candidates {
                if c.pos == end_pos {
                    visited_end = true;
                }
                visited_nodes.insert(c.clone());
                next_nodes.insert(c);
            }
        }
        current_nodes = std::mem::take(&mut next_nodes);
        if visited_end {
            break;
        }
    }
    println!("visited_end: {}", visited_end);

    let mut path = vec![];
    let mut path_next = visited_nodes
        .get(&PathNode {
            pos: end_pos,
            coming_from: None,
            height: 0,
        })
        .unwrap()
        .clone();
    let path_end = PathNode {
        pos: start_pos,
        coming_from: None,
        height: 0,
    };

    let mut counter = 0;
    while path_next != path_end {
        let pos = match path_next.coming_from {
            Some(p) => p,
            None => break,
        };
        let temp = std::mem::replace(
            &mut path_next,
            visited_nodes
                .get(&PathNode {
                    pos: pos,
                    coming_from: None,
                    height: 0,
                })
                .unwrap()
                .clone(),
        );
        path.push(temp);
        counter += 1;
    }
    println!("node: {:?}", path.len());
}
