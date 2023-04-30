use std::collections::HashMap;

fn main() {
    day_one_one();
}

fn day_one_one() {
    let mut elves: HashMap<i32, i32> = HashMap::new();
    let file = std::fs::read_to_string("src/inputs/day_one").unwrap();

    let mut counter = 0;
    file.lines().for_each(|x| {
        if x == "" {
            counter += 1;
        } else {
            let num_x = match x.parse::<i32>() {
                Ok(number) => number,
                Err(..) => 0,
            };
            let curr_val = match elves.get(&counter) {
                Some(val) => val,
                None => &0,
            };

            elves.insert(counter, num_x + curr_val);
        }
    });

    let mut most_calorific_elf = (-1, -1);

    for (key, value) in &elves {
        if value > &most_calorific_elf.1 {
            most_calorific_elf = (*key, *value);
        }
    }

    println!("The highest calories is {}", most_calorific_elf.1);
}