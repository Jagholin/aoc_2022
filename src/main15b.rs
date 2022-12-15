use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

// range from left to right, inclusive
#[derive(Debug)]
struct LineRange(i64, i64);

struct Sensor(i64, i64, u64);

impl PartialEq for LineRange {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl PartialOrd for LineRange {
    /// Returns Some if ranges cannot be unified, or are equal.
    /// Otherwise returns None
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(std::cmp::Ordering::Equal);
        }
        if self.1 + 1 < other.0 {
            return Some(std::cmp::Ordering::Less);
        }
        if self.0 > other.1 + 1 {
            return Some(std::cmp::Ordering::Greater);
        }
        return None;
    }
}

fn range_union(a: &LineRange, b: &LineRange) -> Option<LineRange> {
    let cmr_res = a.partial_cmp(&b);
    match cmr_res {
        Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Greater) => return None,
        _ => {}
    }
    let nx = a.0.min(b.0);
    let ny = a.1.max(b.1);
    Some(LineRange(nx, ny))
}

fn range_intersect(a: &LineRange, b: &LineRange) -> Option<LineRange> {
    let cmr_res = a.partial_cmp(&b);
    match cmr_res {
        Some(std::cmp::Ordering::Less) | Some(std::cmp::Ordering::Greater) => return None,
        _ => {}
    };
    let nx = a.0.max(b.0);
    let ny = a.1.min(b.1);
    if nx <= ny {
        Some(LineRange(nx, ny))
    } else {
        None
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("input15.txt").unwrap();
    let rex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;

    let mut ranges: Vec<LineRange> = vec![];
    let mut critical_beacons = HashSet::new();

    let mut sensors = vec![];

    let critical_area = 4_000_000;

    for line in input.lines() {
        // println!("{}", line);
        let caps = rex.captures(line).unwrap();
        let sx = caps.get(1).unwrap().as_str().parse::<i64>()?;
        let sy = caps.get(2).unwrap().as_str().parse::<i64>()?;
        let bx = caps.get(3).unwrap().as_str().parse::<i64>()?;
        let by = caps.get(4).unwrap().as_str().parse::<i64>()?;

        if bx >= 0 && by >= 0 && bx < critical_area && by < critical_area {
            critical_beacons.insert((bx, by));
        };
        let sensor_radius = bx.abs_diff(sx) + by.abs_diff(sy);
        sensors.push(Sensor(sx, sy, sensor_radius));
    }

    let critical_range = LineRange(0, critical_area - 1);

    for crit_line in 0..critical_area {
        ranges.clear();

        for s in sensors.iter() {
            let projected_radius = s.2 as i64 - s.1.abs_diff(crit_line) as i64;
            if projected_radius < 0 {
                continue;
            }
            // projected center is (sx, critical_line);
            let mut new_range = match range_intersect(
                &critical_range,
                &LineRange(s.0 - projected_radius, s.0 + projected_radius),
            ) {
                Some(r) => r,
                None => continue,
            };
            let mut ranges_left = vec![];
            for r in ranges {
                if let Some(r_unified) = range_union(&r, &new_range) {
                    new_range = r_unified;
                } else {
                    ranges_left.push(r);
                }
            }
            ranges = ranges_left;
            ranges.push(new_range);
        }
        let range_length: i64 = ranges.iter().map(|r| r.1 - r.0 + 1).sum();
        if range_length < critical_area {
            println!(
                "crit line {}, length {}, ranges {:?}",
                crit_line, range_length, ranges
            );
            let mut my_x = 0;
            for test_x in 0..critical_area {
                if ranges.iter().any(|r| test_x >= r.0 && test_x <= r.1) {
                    continue;
                }
                my_x = test_x;
                break;
            }
            println!(
                "x is {}, tuning frequency is {}",
                my_x,
                my_x * 4_000_000 + crit_line
            );
            break;
        }
    }

    Ok(())
}
