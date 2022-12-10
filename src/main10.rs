use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input10.txt").unwrap();
    let mut reg = 1;
    let mut reg_history = vec![];
    for line in input.lines() {
        let mut iter_ws = line.split_whitespace();
        let command = iter_ws.next().unwrap();
        match command {
            "noop" => {
                reg_history.push(reg);
            }
            "addx" => {
                reg_history.push(reg);
                reg_history.push(reg);
                let operand: i32 = iter_ws.next().unwrap().parse().unwrap();
                reg += operand;
            }
            _ => panic!("unknown command {command} in line {line}"),
        }
    }

    let sig_str = 20 * reg_history[19]
        + 60 * reg_history[59]
        + 100 * reg_history[99]
        + 140 * reg_history[139]
        + 180 * reg_history[179]
        + 220 * reg_history[219];
    println!("{sig_str}");

    for vert_line in 0..6 {
        let vert_offset = vert_line * 40;
        for hor_offset in 0..40 {
            let d_offset: i32 = vert_offset + hor_offset;
            let sig_str = reg_history[d_offset as usize];
            if (sig_str - hor_offset).abs() <= 1 {
                print!("#");
            } else {
                print!(".")
            }
        }
        println!("");
    }
}
