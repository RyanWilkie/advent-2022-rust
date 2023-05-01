use std::collections::HashMap;

fn main() {
    let curr_day: u32 = 3;
    match curr_day {
        1 => day_one(3),
        2 => day_two(),
        3 => day_three(),
        _ => println!("Day not implemented"),
    }
}

fn day_one(top_elf_count: u32) {
    let mut elves: Vec<u32> = Vec::new();
    let file = std::fs::read_to_string("src/inputs/day_one").unwrap();

    elves.push(0);
    file.lines().for_each(|x| {
        if x == "" {
            elves.push(0);
        } else {
            let num_x = match x.parse::<u32>() {
                Ok(number) => number,
                Err(..) => 0,
            };
            let curr_val = match elves.pop() {
                Some(number) => number,
                None => 0,
            };

            elves.push(num_x + curr_val);
        }
    });

    elves.sort();

    let mut total_calories: u32 = 0;
    for _n in 1..=top_elf_count {
        let val = match elves.pop() {
            Some(number) => number,
            None => 0,
        };
        total_calories += val;
    }

    println!("total calories {}", total_calories);
}

fn day_two() {
    let mut total_score: u32 = 0;
    // Rock, A, 1
    // Paper, B, 2
    // Scissors, C, 3
    // Win, Z
    // Draw, Y
    // Lose, X
    let matched_attacks = HashMap::from([
        (
            "A",
            HashMap::from([
                ("X", 3),
                ("Y", 1),
                ("Z", 2),
            ]),
        ),
        (
            "B",
            HashMap::from([
                ("X", 1),
                ("Y", 2),
                ("Z", 3),
            ]),
        ),
        (
            "C",
            HashMap::from([
                ("X", 2),
                ("Y", 3),
                ("Z", 1),
            ]),
        ),
    ]);
    let file = std::fs::read_to_string("src/inputs/day_two").unwrap();

    file.lines().for_each(|line| {
        let mut attacks: Vec<&str> = line.split(" ").take(2).collect();
        let my_attack = match attacks.pop() {
            Some(str) => str,
            None => "",
        };
        let opp_attack = match attacks.pop() {
            Some(str) => str,
            None => "",
        };

        match my_attack {
            "X" => {
                total_score += 0;
            }
            "Y" => {
                total_score += 3;
            }
            "Z" => {
                total_score += 6;
            }
            &_ => todo!(),
        }

        match matched_attacks.get(opp_attack) {
            Some(hash) => {
                match hash.get(my_attack) {
                    Some(number) => {
                        total_score += number;
                    }
                    None => println!("failed to find my attack"),
                }
            }
            None => println!("failed to find opponent attack!"),
        }
    });

    println!("My score is {}", total_score);
}

fn day_three() {
    let file = std::fs::read_to_string("src/inputs/day_three").unwrap();
    let letters = String::from_utf8((b'a'..=b'z').chain(b'A'..=b'Z').collect()).unwrap();
    let mut total_priority = 0;

    file.lines().for_each(|x| {
        let (left_compartment, right_compartment) = x.split_at(x.len() / 2);
        let mut matched_char = '\0';
        left_compartment
            .chars()
            .into_iter()
            .for_each(|x| {
                if right_compartment.find(x).is_some() {
                    matched_char = x;
                }
            });

        match letters.find(matched_char) {
            Some(number) => {
                // plus one because index starts at 0, priority starts at 1
                total_priority += number + 1;
            }
            None => println!("Couldn't find letter"),
        }
    });

    println!("{}", total_priority);
}