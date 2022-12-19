use super::monkey_input::MonkeyInput;
use super::types::{MonkeyName, Num};
use anyhow::Result;

pub struct Monkey {
    pub name: MonkeyName,
    pub items: Vec<Num>,
    pub operation: (String, String),
    pub divisible_by: Num,
    pub monkey_if_test_true: MonkeyName,
    pub monkey_if_test_false: MonkeyName,
    pub inspect_counter: usize,
    item_to_throw: Option<Num>,
    test_result: Option<bool>,
}

impl Monkey {
    pub fn new(input: &MonkeyInput) -> Self {
        return Self {
            name: input.name.clone(),
            items: input.starting_items.clone(),
            operation: input.operation.clone(),
            divisible_by: input.divisible_by,
            monkey_if_test_true: input.monkey_if_test_true.clone(),
            monkey_if_test_false: input.monkey_if_test_false.clone(),
            inspect_counter: 0,
            item_to_throw: None,
            test_result: None,
        };
    }

    pub fn update_worry_level(&mut self, item: &mut Num) -> Result<()> {
        let by: Num;

        if self.operation.1.as_str() == "old" {
            by = item.clone();
        } else {
            by = self.operation.1.parse().unwrap();
        }

        match self.operation.0.as_str() {
            "*" => {
                *item = *item * by;
            }
            "+" => {
                *item = *item + by;
            }
            _ => {
                panic!("Unknown operation")
            }
        }
        Ok(())
    }

    pub fn do_monkey_test_for_item(&mut self, item: &mut Num) -> Result<()> {
        self.test_result = Some(*item % self.divisible_by == 0);
        self.item_to_throw = Some(*item);
        Ok(())
    }

    pub fn target_monkey_name(&self) -> Result<MonkeyName> {
        // it is save to clone target monkey name.
        if self.test_result.unwrap() {
            Ok(self.monkey_if_test_true.clone())
        } else {
            Ok(self.monkey_if_test_false.clone())
        }
    }

    pub fn throw(&self) -> Result<Num> {
        Ok(self.item_to_throw.unwrap())
    }
}
