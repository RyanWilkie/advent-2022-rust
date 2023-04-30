fn main() {
    day_one(3);
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