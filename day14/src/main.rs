use std::collections::HashMap;

#[derive(Debug)]
struct ReactionInput {
    id: String,
    count: usize,
}

#[derive(Debug)]
struct Reaction {
    output_count: usize,
    inputs: Vec<ReactionInput>,
}

fn parse_pair(s: &str) -> (usize, &str) {
    let v: Vec<&str> = s.split(' ').collect();
    let count = v[0].parse().unwrap();
    (count, v[1])
}

// ex. 1 A, 2 B, 3 C => 2 D
fn parse_reactions(s: &str) -> HashMap<String, Reaction> {
    s.lines()
        .map(|line| {
            let v: Vec<&str> = line.split(" => ").collect();
            let inputs: Vec<ReactionInput> = v[0]
                .split(", ")
                .map(|i| {
                    let (count, id) = parse_pair(i);
                    ReactionInput {
                        count,
                        id: id.to_owned(),
                    }
                })
                .collect();
            let (output_count, output_id) = parse_pair(v[1]);
            (
                output_id.to_owned(),
                Reaction {
                    output_count,
                    inputs,
                },
            )
        })
        .collect()
}

fn main() {
    let input = include_str!("../input.txt").trim();
    let reactions = parse_reactions(input);
    // what is the minimum amount of ORE required to produce exactly 1 FUEL?
    let mut total = HashMap::<&str, usize>::new();
    let mut used = HashMap::<&str, usize>::new();
    let mut needed_ids = vec!["FUEL"];
    let mut needed_counts = HashMap::new();
    needed_counts.insert("FUEL".to_owned(), 1);
    loop {
        match needed_ids.pop() {
            None => {
                if *total.get("ORE").unwrap() >= 1_000_000_000_000 {
                    break;
                }
                needed_ids.push("FUEL");
                needed_counts.insert("FUEL".to_owned(), 1);
            }
            Some(need_id) => {
                let need_count = needed_counts.remove(need_id).unwrap();
                let e = used.entry(need_id).or_default();
                *e += need_count;
                let reaction = reactions.get(need_id).unwrap();
                let times = if need_count % reaction.output_count == 0 {
                    need_count / reaction.output_count
                } else {
                    need_count / reaction.output_count + 1
                };
                // println!("{}: {}x {:?}", id, times, reaction);
                let e = total.entry(need_id).or_default();
                *e += times * reaction.output_count;
                for input in reaction.inputs.iter() {
                    if input.id == "ORE" {
                        let e = total.entry(&input.id).or_default();
                        *e += times * input.count;
                    } else {
                        let mut required = times * input.count;
                        let t = total.entry(&input.id).or_default();
                        let u = used.entry(&input.id).or_default();
                        let available = *t - *u;
                        if available > required {
                            *u += required;
                        } else {
                            *u += available;
                            required -= available;
                            if needed_counts.contains_key(&input.id) {
                                let n = needed_counts.entry(input.id.clone()).or_default();
                                *n += required;
                            } else {
                                needed_counts.insert(input.id.clone(), required);
                                needed_ids.push(&input.id);
                            }
                        }
                    }
                }
            }
        }
    }
    println!("fuel: {}", total.get("FUEL").unwrap() - 1);
    println!("ore: {}", total.get("ORE").unwrap());
}
