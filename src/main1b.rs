use std::fs::read_to_string;

fn main() {
    println!("Hello, world!");
    let mut elves = vec![];
    let mut max_calories:u32 = 0;
    let mut current_calories = 0;
    let input = read_to_string("input1a.txt").unwrap();
    for mut line in input.lines() {
        line = line.trim();
        let num = line.parse::<u32>();
        match num {
            Ok(calory)  => {
                current_calories += calory;
            },
            Err(_) => {
                // empty line
                if max_calories < current_calories {
                    max_calories = current_calories;
                }
                elves.push(current_calories);
                current_calories = 0;
            }
        }
    }
    if max_calories < current_calories {
        max_calories = current_calories;
    }
    elves.push(current_calories);
    elves.sort();
    // get sum of top 3 elves
    let result = elves.pop().unwrap() + elves.pop().unwrap() + elves.pop().unwrap();
    println!("calories total: {result}");
}