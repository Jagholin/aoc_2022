use std::fs::read_to_string;

fn main() {
    let mut input = read_to_string("input6a.txt").unwrap();
    let vec_inp: Vec<_> = input.chars().collect();
    const WINDOW_SIZE: usize = 14;
    let mut counter = WINDOW_SIZE;
    for window in vec_inp.windows(WINDOW_SIZE) {
        //println!("{:?}", window);
        let mut buff = vec![];
        let mut repeat = false;
        for ch in window {
            if buff.contains(ch) {
                repeat = true;
                break;
            }
            buff.push(*ch);
        }
        if !repeat {
            println!("final counter: {counter}");
            break;
        }
        counter += 1;
    }
}