use std::io::Error;

pub mod common;

fn get_priority(item: &char) -> u32 {
    return match item.is_uppercase() {
        true => *item as u32 - 38,
        false => *item as u32 - 96,
    };
}

fn get_priority_for_common_item(elve_group: &(&str, &str, &str)) -> u32 {
    for item_a in elve_group.0.chars() {
        for item_b in elve_group.1.chars() {
            for item_c in elve_group.2.chars() {
                if item_a == item_b && item_a == item_c {
                    let priority = get_priority(&item_a);
                    println!("Common rucksack item: {} with prio: {}", item_a, priority);
                    return priority;
                }
            }
        }
    }
    return 0;
}

fn group_elves(rucksacks: &Vec<Result<String, Error>>) -> Vec<(&str, &str, &str)> {
    let mut offset = 0;
    let mut elve_groups: Vec<(&str, &str, &str)> = Vec::new();

    while offset <= rucksacks.len() - 3 {
        elve_groups.push((
            rucksacks[offset].as_ref().unwrap(),
            rucksacks[offset + 1].as_ref().unwrap(),
            rucksacks[offset + 2].as_ref().unwrap(),
        ));
        offset = offset + 3;
    }
    return elve_groups;
}

fn main() {
    let args = common::read_args();
    let input_file = common::read_file(&args[1]);

    let elve_groups = group_elves(&input_file);
    let mut priority_sum = 0;

    for group in elve_groups {
        let prio = get_priority_for_common_item(&group);
        priority_sum = priority_sum + prio;
    }
    println!("Sum of priorities: {}", priority_sum);
}
