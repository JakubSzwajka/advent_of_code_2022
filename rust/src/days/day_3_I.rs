pub mod common;

fn get_priority(item: &char) -> u32 {
    return match item.is_uppercase() {
        true => *item as u32 - 38,
        false => *item as u32 - 96,
    };
}

fn get_priority_for_common_item(first_compartment: &[char], second_compartment: &[char]) -> u32 {
    for item_a in first_compartment {
        for item_b in second_compartment {
            if item_a == item_b {
                let priority = get_priority(item_a);
                println!("Common rucksack item: {} with prio: {}", item_a, priority);
                return priority;
            }
        }
    }
    return 0;
}

fn main() {
    let args = common::read_args();
    let input_file = common::read_file(&args[1]);

    let mut priority_sum = 0;

    for row in input_file {
        match row {
            Ok(n) => {
                let rucksack_items: Vec<char> = n.chars().collect();
                let first_compartment = &rucksack_items[0..&rucksack_items.len() / 2];
                let second_compartment =
                    &rucksack_items[&rucksack_items.len() / 2..rucksack_items.len()];

                priority_sum = priority_sum
                    + get_priority_for_common_item(&first_compartment, &second_compartment);
            }

            Err(e) => {
                println!("Error reading line: {}", e)
            }
        }
    }

    println!("Sum of priorities: {}", priority_sum);
}
