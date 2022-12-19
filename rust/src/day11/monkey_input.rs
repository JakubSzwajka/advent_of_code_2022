use super::types::{MonkeyName, Num};

#[derive(Debug)]
pub struct MonkeyInput {
    pub name: MonkeyName,
    pub starting_items: Vec<Num>,
    pub operation: (String, String),
    pub divisible_by: Num,
    pub monkey_if_test_true: u32,
    pub monkey_if_test_false: u32,
}

impl MonkeyInput {
    pub fn new(data: &str) -> MonkeyInput {
        let data = data.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();

        MonkeyInput {
            name: MonkeyInput::get_name(data[0]),
            starting_items: MonkeyInput::get_starting_items(data[1]),
            operation: MonkeyInput::get_operation(data[2]),
            divisible_by: MonkeyInput::get_test_val(data[3]),
            monkey_if_test_true: MonkeyInput::get_target_monkey(data[4]),
            monkey_if_test_false: MonkeyInput::get_target_monkey(data[5]),
        }
    }

    fn get_name(data: &str) -> String {
        let mut name = data.split(" ").collect::<Vec<&str>>()[1].to_string();
        name.pop();
        return name;
    }

    fn get_starting_items(data: &str) -> Vec<Num> {
        return data.split(':').collect::<Vec<&str>>()[1]
            .split(',')
            .map(|x| x.to_string().trim().parse().unwrap())
            .collect::<Vec<Num>>();
    }

    fn get_test_val(data: &str) -> Num {
        return data.split("by").collect::<Vec<&str>>()[1]
            .to_string()
            .trim()
            .parse()
            .unwrap();
    }

    fn get_operation(data: &str) -> (String, String) {
        let operation_data = data.split("new = old").collect::<Vec<&str>>()[1]
            .split(" ")
            .collect::<Vec<&str>>();
        return (operation_data[1].to_string(), operation_data[2].to_string());
    }

    fn get_target_monkey(data: &str) -> u32 {
        return data
            .chars()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .pop()
            .unwrap()
            .parse()
            .unwrap();
    }
}
