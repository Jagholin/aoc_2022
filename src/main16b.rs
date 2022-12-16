use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: u32,
    connections: Vec<&'a str>,
}

#[derive(Debug)]
struct ValvePaths<'a> {
    flow_rate: u32,
    connections: Vec<&'a str>,
    path_lengths: HashMap<&'a str, u32>,
}

#[derive(Debug)]
struct PathElement<'a> {
    value: u32, // value of the current valve
    // total_value: u32,
    path_length: usize, // how much time is lost getting to the node
    node: &'a str,
}

#[derive(Debug, Clone)]
struct Agent<'a> {
    position: &'a str,
    remaining_time: i32,
}

fn fill_path_lengths<'a>(
    valves_graph: HashMap<&'a str, Valve<'a>>,
) -> HashMap<&'a str, ValvePaths<'a>> {
    let mut result = HashMap::new();

    for (name, v) in valves_graph.iter() {
        let mut p_lengths = HashMap::new();
        let mut next_steps = v.connections.clone();
        let mut next_next_steps = vec![];
        let mut current_path_length = 1;

        while !next_steps.is_empty() {
            for next_step in next_steps {
                if p_lengths.contains_key(next_step) || next_step == *name {
                    continue;
                }
                p_lengths.insert(next_step, current_path_length);
                for c in valves_graph.get(next_step).unwrap().connections.iter() {
                    if next_next_steps.contains(c) {
                        continue;
                    }
                    next_next_steps.push(*c);
                }
            }
            current_path_length += 1;
            next_steps = std::mem::take(&mut next_next_steps);
        }

        result.insert(
            *name,
            ValvePaths {
                flow_rate: v.flow_rate,
                connections: v.connections.clone(),
                path_lengths: p_lengths,
            },
        );
    }
    result
}

fn optimize_path<'a>(
    mut agents: Vec<Agent>,
    valves_graph: &HashMap<&str, ValvePaths>,
    remaining_destinations: Vec<&'a str>,
    alpha_factor: u32,
    current_value: u32,
) -> Option<(Vec<PathElement<'a>>, u32)> {
    // first figure out limit of what we can possibly do with remaining destinations and time
    // if we have less than 2 minutes, nothing we can possibly do will help
    if agents.iter().all(|a| a.remaining_time <= 2) {
        if current_value > alpha_factor {
            return Some((vec![], current_value));
        }
        return None;
    }

    // value ceiling that we can possibly get. Real values will be much lower.
    let max_value = remaining_destinations
        .iter()
        .map(|d| {
            agents
                .iter()
                .map(|a| {
                    let valve = valves_graph.get(*d).unwrap();
                    let time_needed = *valves_graph
                        .get(a.position)
                        .unwrap()
                        .path_lengths
                        .get(*d)
                        .unwrap() as i32
                        + 1;
                    if time_needed >= a.remaining_time {
                        0
                    } else {
                        valve.flow_rate * (a.remaining_time - time_needed) as u32
                    }
                })
                .max()
                .unwrap()
        })
        .sum::<u32>()
        + current_value;

    // if we cant be better than the best known path in this iteration, just give up and
    // try something else
    if max_value < alpha_factor {
        return None;
    }

    if remaining_destinations.is_empty() {
        // we have visited all the nodes
        if current_value > alpha_factor {
            return Some((vec![], current_value));
        }
        return None;
    }

    let mut next_alpha_factor = alpha_factor;
    // empty array is still an option if the initial value is bigger than alpha
    let mut candidate = if current_value > alpha_factor {
        Some((vec![], current_value))
    } else {
        None
    };
    agents.sort_unstable_by(|left, right| right.remaining_time.cmp(&left.remaining_time));

    // start checking different paths through the remaining_destinations
    // they are already sorted from highest flow rate to lowest
    for agent_id in 0..agents.len() {
        let mut agent_moved = false;

        for (i, dest) in remaining_destinations.iter().enumerate() {
            // search path from the current node to this one
            let Some(path) = valves_graph.get(agents[agent_id].position).unwrap().path_lengths.get(dest) else {
                continue;
            };
            // each one of the nodes in path takes 1 min, then 1 min to open the valve
            let rem_time = agents[agent_id].remaining_time - *path as i32 - 1;
            if rem_time < 0 {
                continue;
            }
            let new_remaining_destinations: Vec<_> = remaining_destinations
                .iter()
                .filter_map(|p| if *p != *dest { Some(*p) } else { None })
                .collect();
            let valve_value = rem_time as u32 * valves_graph.get(*dest).unwrap().flow_rate;
            let value = current_value + valve_value;

            let mut new_agents = agents.clone();
            new_agents[0] = Agent {
                position: *dest,
                remaining_time: rem_time,
            };

            let Some(mut tail) = optimize_path( new_agents, &valves_graph, new_remaining_destinations, next_alpha_factor, value) else {
                continue;
            };

            next_alpha_factor = tail.1;

            tail.0.push(PathElement {
                node: *dest,
                path_length: *path as usize,
                value: valve_value,
            });

            candidate = Some(tail);
            agent_moved = true;
        }
        if agent_moved {
            break;
        }
    }
    candidate
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string("input16.txt").unwrap();
    let line_rex = Regex::new(r"Valve (\w+) has flow rate=(\d+); tunnels? leads? to valves? (.+)")?;

    let mut valves = HashMap::new();
    let mut destinations = vec![];

    for line in input.lines() {
        let Some(caps) = line_rex.captures(line) else {
            println!("Cannot parse the line {}", line);
            return Ok(());
        };

        let valve_name = caps.get(1).unwrap().as_str();
        let valve_flow = caps.get(2).unwrap().as_str().parse::<u32>()?;
        let connections = caps.get(3).unwrap().as_str();

        valves.insert(
            valve_name,
            Valve {
                flow_rate: valve_flow,
                connections: connections.split(", ").collect(),
            },
        );

        if valve_flow > 0 {
            // the valve is a possible destination
            destinations.push(valve_name);
        }
    }

    let valves = fill_path_lengths(valves);

    // sort destinations vec by flow rate, starting with the highest one

    destinations.sort_by(|left, right| {
        let l = valves.get(*left).unwrap();
        let r = valves.get(*right).unwrap();
        r.flow_rate.cmp(&l.flow_rate)
    });

    // println!("valves paths {:?}", valves);
    println!("destinations {:?}", destinations);

    let agents = vec![
        Agent {
            position: "AA",
            remaining_time: 26,
        },
        Agent {
            position: "AA",
            remaining_time: 26,
        },
    ];

    let optimal_path = optimize_path(agents, &valves, destinations, 0, 0);

    println!("optimal path data: {:?}", optimal_path);

    Ok(())
}
