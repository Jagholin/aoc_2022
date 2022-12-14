use regex::Regex;
use std::{error::Error, fs::read_to_string};

fn bounds_check(pos: &(isize, isize), width: usize, height: usize) -> bool {
    pos.0 >= 0 && pos.1 >= 0 && pos.0 < width as isize && pos.1 < height as isize
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = read_to_string("input14.txt").unwrap();
    let coord_rex = Regex::new(r"(\d+),(\d+)")?;

    let mut min_x = 500;
    let mut min_y = 0;

    let mut max_x = 500;
    let mut max_y = 0;

    let mut rock_coords = vec![];

    for line in input.lines() {
        let mut prev_coord: Option<(isize, isize)> = None;
        for coord in line.split("->") {
            let caps = coord_rex.captures(coord).unwrap();
            let x = caps.get(1).unwrap().as_str().parse::<usize>()?;
            let y = caps.get(2).unwrap().as_str().parse::<usize>()?;
            min_x = x.min(min_x);
            min_y = y.min(min_y);
            max_x = x.max(max_x);
            max_y = y.max(max_y);

            if let Some(c) = prev_coord {
                let dx = (x as isize - c.0 as isize).signum();
                let dy = (y as isize - c.1 as isize).signum();

                let mut current_pos = c;
                while current_pos != (x as isize, y as isize) {
                    current_pos.0 += dx;
                    current_pos.1 += dy;
                    rock_coords.push(current_pos);
                }
            } else {
                // start of the line, put a rock there
                rock_coords.push((x as isize, y as isize))
            }
            prev_coord = Some((x as isize, y as isize));
        }
    }

    // create a 2dim nested vec that encompasses the board
    // offset by (min_x, min_y)

    let min_x = min_x - 200;
    let max_x = max_x + 200;
    let min_y = min_y;

    println!("{} {} - {} {}", min_x, min_y, max_x, max_y);
    println!("{}", rock_coords.len());

    let height = max_y - min_y + 3;
    let width = max_x - min_x + 1;

    let emitter_pos = (500 - min_x as isize, 0 - min_y as isize);

    let mut board = vec![];
    for _ in 0..width {
        let mut inner_vec = vec![0; height - 1];
        inner_vec.push(1);
        board.push(inner_vec);
    }

    for rock in rock_coords {
        board[rock.0 as usize - min_x][rock.1 as usize - min_y] = 1;
    }

    let mut sand_particles_at_rest = 0;

    'outer: loop {
        let mut active_sand_pos = emitter_pos;

        if board[active_sand_pos.0 as usize][active_sand_pos.1 as usize] != 0 {
            println!("finished");
            break;
        }

        loop {
            // try move down
            let next_pos = (active_sand_pos.0, active_sand_pos.1 + 1);
            if !bounds_check(&next_pos, width, height) {
                // sand escapes
                println!("ESCAPE");
                break 'outer;
            }
            if board[next_pos.0 as usize][next_pos.1 as usize] == 0 {
                // is free
                active_sand_pos = next_pos;
                continue;
            }

            // try move down left
            let next_pos = (active_sand_pos.0 - 1, active_sand_pos.1 + 1);
            if !bounds_check(&next_pos, width, height) {
                // sand escapes
                println!("ESCAPE LEFT");
                break 'outer;
            }
            if board[next_pos.0 as usize][next_pos.1 as usize] == 0 {
                // is free
                active_sand_pos = next_pos;
                continue;
            }

            // and down right
            let next_pos = (active_sand_pos.0 + 1, active_sand_pos.1 + 1);
            if !bounds_check(&next_pos, width, height) {
                // sand escapes
                println!("ESCAPE RIGHT");
                break 'outer;
            }
            if board[next_pos.0 as usize][next_pos.1 as usize] == 0 {
                // is free
                active_sand_pos = next_pos;
                continue;
            }

            // cant move anywhere, stop here
            board[active_sand_pos.0 as usize][active_sand_pos.1 as usize] = 2;
            sand_particles_at_rest += 1;
            break;
        }
    }

    // for i in 0..height {
    //     for j in 0..width {
    //         if board[j][i] == 1 {
    //             print!("#");
    //         } else if board[j][i] == 2 {
    //             print!("o");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!("");
    // }

    println!("sand particles at rest {}", sand_particles_at_rest);

    Ok(())
}
