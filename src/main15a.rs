use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

// range from left to right, inclusive
#[derive(Debug)]
struct LineRange(i64, i64);

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

fn beacon_inside(a: &LineRange, b: i64) -> bool {
    b >= a.0 && b <= a.1
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("input15.txt").unwrap();
    let rex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")?;

    let mut ranges: Vec<LineRange> = vec![];
    let mut critical_beacons = HashSet::new();

    let critical_line = 2_000_000;

    for line in input.lines() {
        // println!("{}", line);
        let caps = rex.captures(line).unwrap();
        let sx = caps.get(1).unwrap().as_str().parse::<i64>()?;
        let sy = caps.get(2).unwrap().as_str().parse::<i64>()?;
        let bx = caps.get(3).unwrap().as_str().parse::<i64>()?;
        let by = caps.get(4).unwrap().as_str().parse::<i64>()?;

        if by == critical_line {
            critical_beacons.insert(bx);
        }

        let sensor_radius = bx.abs_diff(sx) + by.abs_diff(sy);
        let projected_radius = sensor_radius as i64 - sy.abs_diff(critical_line) as i64;
        if projected_radius < 0 {
            continue;
        }

        // projected center is (sx, critical_line);
        let mut new_range = LineRange(sx - projected_radius, sx + projected_radius);
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

    println!("ranges {:?}", ranges);
    let range_length: i64 = ranges.iter().map(|r| r.1 - r.0 + 1).sum();
    let crit_beacon_count = critical_beacons
        .into_iter()
        .filter(|v| ranges.iter().any(|r| beacon_inside(r, *v)))
        .count() as i64;
    println!("ranges total length: {}", range_length - crit_beacon_count);
    Ok(())
}
