use std::fs::read_to_string;
use std::cmp::max;

fn tree_visibility_from(mut i: usize, mut j: usize, treemap: &Vec<Vec<u8>>, di: isize, dj: isize, H: usize, W: usize) -> i32 {
    let mut visibility = 0;
    let compare = treemap[j][i];
    // println!("position {i} {j} in direction {di} {dj}, treemap {compare}: ");
    while i != 0 && j != 0 && i != W-1 && j != H-1 {
        // println!("adding {di} to {i}, and {dj} to {j}");
        i = i.wrapping_add(di as usize);
        j = j.wrapping_add(dj as usize);
        // println!("    treemap {i} {j}: {}", treemap[j][i]);
        if treemap[j][i] < compare {
            visibility += 1;
        }
        if treemap[j][i] >= compare {
            visibility += 1;
            break;
        }
    }
    // println!("visibility {visibility}");
    visibility
}

fn tree_visibility(i: usize, j: usize, treemap: &Vec<Vec<u8>>, H: usize, W: usize) -> i32 {
    if i == 0 || j == 0 || i == W-1 || j == H-1 {
        // trivial case
        return 0;
    }
    tree_visibility_from(i, j, treemap, -1, 0, H, W) * tree_visibility_from(i, j, treemap, 1, 0, H, W) 
        * tree_visibility_from(i, j, treemap, 0, -1, H, W) * tree_visibility_from(i, j, treemap, 0, 1, H, W)
}

fn main() {
    let input = read_to_string("input8.txt").unwrap();
    let mut treeline: Vec<Vec<u8>> = vec![];
    for line in input.lines() {
        let mut trees: Vec<u8> = vec![];
        for c in line.chars() {
            trees.push(c.to_digit(10).unwrap() as u8);
        }
        treeline.push(trees);
    }
    let treeline = treeline;
    let H = treeline.len();
    let W = treeline[0].len();
    let mut counter = 0;
    for i in 0..W {
        for j in 0..H {
            counter = max(counter, tree_visibility(i, j, &treeline, H, W));
        }
    }
    println!("{counter}");
}
