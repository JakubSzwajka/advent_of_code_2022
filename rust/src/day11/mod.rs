pub mod monkey;
pub mod monkey_input;
pub mod types;

use crate::solution::{Solution, SolutionInput};
use anyhow::{Ok, Result};
use monkey::Monkey;
use monkey_input::MonkeyInput;
use types::Num;

fn get_least_common_divisor(input: &[MonkeyInput]) -> u64 {
    // least common multiply / l    est common divisor
    input
        .iter()
        .map(|m| m.divisible_by)
        .collect::<Vec<u64>>()
        .iter()
        .product()
}

fn do_round<F: FnMut(&mut Num)>(monkeys: &mut Vec<Monkey>, operation: &mut F) -> Result<()> {
    for i in 0..monkeys.len() {
        do_turn(monkeys, i, operation)?;
    }
    Ok(())
}

fn do_turn<F: FnMut(&mut Num)>(
    monkeys: &mut Vec<Monkey>,
    name: usize,
    operation: &mut F,
) -> Result<()> {
    monkeys[name].inspect_counter += monkeys[name].items.len();
    for _j in 0..monkeys[name].items.len() {
        let mut item = monkeys[name].items.remove(0);
        monkeys[name].update_worry_level(&mut item)?;
        operation(&mut item);
        monkeys[name].do_monkey_test_for_item(&mut item)?;
        let target_monkey_name = monkeys[name].target_monkey_name()?;
        let thrown_item = monkeys[name].throw()?;
        monkeys[target_monkey_name].items.push(thrown_item);
    }

    Ok(())
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

        for _ in 0..rounds {
            do_round(&mut monkeys, &mut |item: &mut Num| *item /= 3)?;
        }

        monkeys.sort_by(|a, b| b.inspect_counter.cmp(&a.inspect_counter));
        Ok(monkeys[0].inspect_counter * monkeys[1].inspect_counter)
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

        let rounds = 10000;
        let lcd = get_least_common_divisor(input);

        for _i in 0..rounds {
            do_round(&mut monkeys, &mut |item: &mut Num| *item %= lcd)?;
        }

        monkeys.sort_by(|a, b| b.inspect_counter.cmp(&a.inspect_counter));
        Ok(monkeys[0].inspect_counter * monkeys[1].inspect_counter)
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

    #[test]
    fn test_part2_test() -> Result<()> {
        assert_eq!(2713310158, Day11Pt2::solve(&INPUT_TEST)?);
        Ok(())
    }
    #[test]
    fn test_part2_result() -> Result<()> {
        assert_eq!(29703395016, Day11Pt2::solve(&INPUT_MAIN)?);
        Ok(())
    }
}
