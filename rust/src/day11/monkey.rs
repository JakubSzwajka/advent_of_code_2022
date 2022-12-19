use super::monkey_input::MonkeyInput;
use super::types::{MonkeyName, Num};

pub struct Monkey {
    pub name: MonkeyName,
    pub items: Vec<Num>,
    pub operation: (String, String),
    pub divisible_by: Num,
    pub monkey_if_test_true: u32,
    pub monkey_if_test_false: u32,
    pub inspect_counter: u32,
}

impl Monkey {
    pub fn new(input: &MonkeyInput) -> Self {
        return Self {
            name: input.name.clone(),
            items: input.starting_items.clone(),
            operation: input.operation.clone(),
            divisible_by: input.divisible_by,
            monkey_if_test_true: input.monkey_if_test_true,
            monkey_if_test_false: input.monkey_if_test_false,
            inspect_counter: 0,
        };
    }

    fn update_worry_level(&mut self, item: &Num) -> Num {
        let operation: String;
        let new_worry_level: Num;
        let by: Num;

        if self.operation.1.as_str() == "old" {
            by = item.clone();
        } else {
            by = self.operation.1.parse().unwrap();
        }

        match self.operation.0.as_str() {
            "*" => {
                operation = "multiplied".to_string();
                new_worry_level = item * by.clone();
                //
            }
            "+" => {
                operation = "increased".to_string();
                new_worry_level = item + by.clone();
                //
            }
            _ => {
                panic!("Unknown operation")
            }
        }
        println!(
            "  Worry level is {} by {} to {}",
            &operation, &by, &new_worry_level
        );
        return new_worry_level;
    }

    fn monkey_gets_bored(&mut self, item: Num) -> Num {
        let item: Num = item / 3;
        println!(
            "  Monkey gets bored with item. Worry level is divided by 3 to {}",
            item
        );
        return item;
    }

    fn do_monkey_test(&self, item: &Num) -> bool {
        // let test: Num = (item % &self.divisible_by).try_into().unwrap();
        if item % self.divisible_by == 0 {
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

    pub fn inspect_item_and_throw(&mut self) -> (Num, u32) {
        let mut item = self.items.remove(0);
        println!("Monkey inspects an item with a worry level of {}", &item);
        self.inspect_counter = self.inspect_counter + 1;
        item = self.update_worry_level(&item);
        item = self.monkey_gets_bored(item);

        if self.do_monkey_test(&item) {
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

    pub fn catch(&mut self, item: Num) -> () {
        self.items.push(item)
    }
}
