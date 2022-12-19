pub mod monkey;
pub mod monkey_input;
pub mod types;

use crate::solution::{Solution, SolutionInput};
use monkey::Monkey;
use monkey_input::MonkeyInput;
// extern crate num;
use anyhow::{Ok, Result};

fn round(monkeys: &mut Vec<Monkey>) -> () {
    for i in 0..monkeys.len() {
        println!("Monkey {}:", monkeys[i].name);
        for _j in 0..monkeys[i].items.len() {
            let (item, catching_monkey) = monkeys[i].inspect_item_and_throw();
            monkeys[catching_monkey as usize].catch(item);
        }
    }
}

fn print_items(monkeys: &Vec<Monkey>) -> () {
    for monkey in monkeys {
        println!(
            "Monkey {}: {}",
            monkey.name,
            monkey
                .items
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

fn print_inspect_counters(monkeys: &Vec<Monkey>) -> () {
    for monkey in monkeys {
        println!(
            "Monkey {} inspected items {} times",
            monkey.name, monkey.inspect_counter
        )
    }
}

impl SolutionInput for Vec<MonkeyInput> {
    fn parse(input_str: &str) -> Result<Self> {
        let monkeys: Vec<MonkeyInput> = input_str
            .split("\n\n")
            .map(|x| MonkeyInput::new(x))
            .collect();
        Ok(monkeys)
    }
}
pub struct Day11Pt1;

impl Solution for Day11Pt1 {
    const DAY: usize = 11;
    const PART: usize = 2;

    type TInput = Vec<MonkeyInput>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut monkeys = input.iter().map(|m| Monkey::new(m)).collect();
        let rounds = 20;

        for i in 0..rounds {
            println!("Round {}/{}", i, rounds);
            round(&mut monkeys);
            print_items(&monkeys);
        }

        print_inspect_counters(&monkeys);
        monkeys.sort_by(|a, b| b.inspect_counter.cmp(&a.inspect_counter));

        let monkey_business = monkeys[0].inspect_counter * monkeys[1].inspect_counter;
        println!("Monkey business is: {}", monkey_business);

        Ok(monkey_business as usize)
    }
}

pub struct Day11Pt2;

impl Solution for Day11Pt2 {
    const DAY: usize = 11;
    const PART: usize = 2;

    type TInput = Vec<MonkeyInput>;
    type TOutput = usize;

    fn solve(input: &Self::TInput) -> Result<Self::TOutput> {
        let mut monkeys = input.iter().map(|m| Monkey::new(m)).collect();
        let rounds = 20;

        for i in 0..20 {
            // println!("Round {}/{}", i, rounds);
            round(&mut monkeys);
            // print_items(&monkeys);
        }

        print_inspect_counters(&monkeys);
        monkeys.sort_by(|a, b| b.inspect_counter.cmp(&a.inspect_counter));

        let monkey_business = monkeys[0].inspect_counter * monkeys[1].inspect_counter;
        println!("Monkey business is: {}", monkey_business);

        Ok(monkey_business as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::common::get_input;
    use lazy_static::lazy_static;

    lazy_static! {
        static ref INPUT_TEST: Vec<MonkeyInput> = get_input::<Day11Pt2>("test.txt").unwrap();
        static ref INPUT_MAIN: Vec<MonkeyInput> = get_input::<Day11Pt2>("input.txt").unwrap();
    }
    #[test]
    fn test_part1_test() -> Result<()> {
        assert_eq!(10605, Day11Pt1::solve(&INPUT_TEST)?);
        Ok(())
    }

    #[test]
    fn test_part1_result() -> Result<()> {
        assert_eq!(113232, Day11Pt1::solve(&INPUT_MAIN)?);
        Ok(())
    }
}
