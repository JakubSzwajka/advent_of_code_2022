pub mod common;
extern crate num;
use num::bigint::{BigInt, Sign};

#[derive(Debug)]
struct Monkey {
    name: String,
    items: Vec<u128>,
    operation: (String, String),
    divisible_by: u128,
    monkey_if_test_true: u32,
    monkey_if_test_false: u32,
    inspect_counter: u32,
}

impl Monkey {
    fn new(data: &str) -> Monkey {
        let data = data.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();
        // dbg!(&data);
        Monkey {
            name: Monkey::_get_name(data[0]),
            items: Monkey::_get_starting_items(data[1]),
            operation: Monkey::_get_operation(data[2]),
            divisible_by: Monkey::_get_test_val(data[3]),
            monkey_if_test_true: Monkey::_get_target_monkey(data[4]),
            monkey_if_test_false: Monkey::_get_target_monkey(data[5]),
            inspect_counter: 0,
        }
    }

    fn _get_name(data: &str) -> String {
        let mut name = data.split(" ").collect::<Vec<&str>>()[1].to_string();
        name.pop();
        return name;
    }

    fn _get_starting_items(data: &str) -> Vec<u128> {
        return data.split(':').collect::<Vec<&str>>()[1]
            .split(',')
            .map(|x| x.to_string().trim().parse().unwrap())
            .collect::<Vec<u128>>();
    }

    fn _get_test_val(data: &str) -> u128 {
        return data.split("by").collect::<Vec<&str>>()[1]
            .to_string()
            .trim()
            .parse()
            .unwrap();
    }

    fn _get_operation(data: &str) -> (String, String) {
        let operation_data = data.split("new = old").collect::<Vec<&str>>()[1]
            .split(" ")
            .collect::<Vec<&str>>();
        return (operation_data[1].to_string(), operation_data[2].to_string());
    }

    fn _get_target_monkey(data: &str) -> u32 {
        return data
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .pop()
            .unwrap()
            .parse()
            .unwrap();
    }

    fn _update_worry_level(&mut self, item: u128) -> u128 {
        let operation: String;
        let new_worry_level: u128;
        let by: u128;

        if self.operation.1.as_str() == "old" {
            by = item;
        } else {
            by = self.operation.1.parse().unwrap();
        }

        match self.operation.0.as_str() {
            "*" => {
                operation = "multiplied".to_string();
                new_worry_level = item * by;
                //
            }
            "+" => {
                operation = "increased".to_string();
                new_worry_level = item + by;
                //
            }
            _ => {
                panic!("Unknown operation")
            }
        }
        println!(
            "  Worry level is {} by {} to {}",
            operation, by, new_worry_level
        );
        return new_worry_level;
    }

    fn _monkey_gets_bored(&mut self, item: u128) -> u128 {
        let item: u128 = item / 3;
        println!(
            "  Monkey gets bored with item. Worry level is divided by 3 to {}",
            item
        );
        return item;
    }

    fn _do_monkey_test(&self, item: u128) -> bool {
        let test = item % self.divisible_by == 0;

        if test {
            println!(
                "  Current worry level is divisible by {}",
                self.divisible_by
            );
            return true;
        } else {
            println!(
                "  Current worry level is not divisible by {}",
                self.divisible_by
            );
            return false;
        }
    }

    fn inspect_item_and_throw(&mut self) -> (u128, u32) {
        let mut item = self.items.remove(0);
        println!("Monkey inspects an item with a worry level of {}", &item);
        self.inspect_counter = self.inspect_counter + 1;
        item = self._update_worry_level(item);
        // item = self._monkey_gets_bored(item);

        if self._do_monkey_test(item) {
            println!(
                "  Item with worry level {} is thrown to monkey {}",
                item, self.monkey_if_test_true
            );

            return (item, self.monkey_if_test_true);
        } else {
            println!(
                "  Item with worry level {} is thrown to monkey {}",
                item, self.monkey_if_test_false
            );
            return (item, self.monkey_if_test_false);
        }
    }

    fn catch(&mut self, item: u128) -> () {
        self.items.push(item)
    }
}

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

fn main() {
    let args = common::read_args();
    let data = common::read_file(&args[1]).unwrap();

    let rounds = 10000;

    let mut monkeys: Vec<Monkey> = data.split("\n\n").map(|x| Monkey::new(x)).collect();

    for _ in 0..rounds {
        round(&mut monkeys);
        print_items(&monkeys);
    }

    print_inspect_counters(&monkeys);
    monkeys.sort_by(|a, b| b.inspect_counter.cmp(&a.inspect_counter));

    let monkey_business = monkeys[0].inspect_counter * monkeys[1].inspect_counter;
    println!("Monkey counter is: {}", monkey_business);
}
