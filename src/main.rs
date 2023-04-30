use std::{ collections::{ HashMap, HashSet }, hash::Hash };

fn main() {
    // day_one(3);
    day_two();
}

fn day_one(top_elf_count: i32) {
    let mut elves = Vec::new();
    let file = std::fs::read_to_string("src/inputs/day_one").unwrap();

    elves.push(0);
    file.lines().for_each(|x| {
        if x == "" {
            elves.push(0);
        } else {
            let num_x = match x.parse::<i32>() {
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

    let mut total_calories = 0;
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
    let mut total_score = 0;
    // Rock, A, X
    // Paper, B, Y
    // Scissors, C, Z
    let matched_attacks = HashMap::from([
        (
            "A",
            HashMap::from([
                ("X", 3),
                ("Y", 6),
                ("Z", 0),
            ]),
        ),
        (
            "B",
            HashMap::from([
                ("X", 0),
                ("Y", 3),
                ("Z", 6),
            ]),
        ),
        (
            "C",
            HashMap::from([
                ("X", 6),
                ("Y", 0),
                ("Z", 3),
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
                total_score += 1;
            }
            "Y" => {
                total_score += 2;
            }
            "Z" => {
                total_score += 3;
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