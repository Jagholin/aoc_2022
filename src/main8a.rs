use std::fs::read_to_string;

fn tree_visible_from(
    mut i: usize,
    mut j: usize,
    treemap: &Vec<Vec<u8>>,
    di: isize,
    dj: isize,
    H: usize,
    W: usize,
) -> bool {
    let mut visible = true;
    let compare = treemap[j][i];
    while i != 0 && j != 0 && i != W - 1 && j != H - 1 {
        i = i.wrapping_add(di as usize);
        j = j.wrapping_add(dj as usize);
        if treemap[j][i] >= compare {
            visible = false;
            break;
        }
    }
    visible
}

fn tree_visible(i: usize, j: usize, treemap: &Vec<Vec<u8>>, H: usize, W: usize) -> bool {
    if i == 0 || j == 0 || i == W - 1 || j == H - 1 {
        // trivial case
        return true;
    }
    tree_visible_from(i, j, treemap, -1, 0, H, W)
        || tree_visible_from(i, j, treemap, 1, 0, H, W)
        || tree_visible_from(i, j, treemap, 0, -1, H, W)
        || tree_visible_from(i, j, treemap, 0, 1, H, W)
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
            counter += if tree_visible(i, j, &treeline, H, W) {
                1
            } else {
                0
            };
        }
    }
    println!("{counter}");
}
