use std::str::FromStr;

pub mod common;

// A - Rock --> 1
// B - Paper --> 2
// C - Scissors --> 3

// Response
// X - Rock
// Y - Paper
// C - Scissors

// Total score = sum(scores in all rounds)
// Round score = shape + outcome ( 0 - lost, 3 - draw , 6 - win)

enum FightResult {
    LOST,
    DRAW,
    WIN,
}

enum OpponentChoices {
    A(String),
    B(String),
    C(String),
}

#[derive(Clone)]
enum MyChoices {
    X(String),
    Y(String),
    Z(String),
}

fn get_points_for_result(result: FightResult) -> u8 {
    return match result {
        FightResult::LOST => 0,
        FightResult::DRAW => 3,
        FightResult::WIN => 6,
    };
}

fn get_points_for_my_choose(_my_choose: &MyChoices) -> u8 {
    return match _my_choose {
        MyChoices::X(_my_choose) => 1,
        MyChoices::Y(_my_choose) => 2,
        MyChoices::Z(_my_choose) => 3,
    };
}

fn get_choices(choices: Vec<&str>) -> (OpponentChoices, MyChoices) {
    let opponent_choice = match choices[0].as_ref() {
        "A" => OpponentChoices::A(choices[0].to_string()),
        "B" => OpponentChoices::B(choices[0].to_string()),
        "C" => OpponentChoices::C(choices[0].to_string()),
        _ => panic!(),
    };

    let my_choice = match choices[1].as_ref() {
        "X" => MyChoices::X(choices[1].to_string()),
        "Y" => MyChoices::Y(choices[1].to_string()),
        "Z" => MyChoices::Z(choices[1].to_string()),
        _ => panic!(),
    };

    return (opponent_choice, my_choice);
}

fn fight(opponent_choose: &OpponentChoices, my_choose: &MyChoices) -> u8 {
    let fight_result: FightResult = match opponent_choose {
        OpponentChoices::A(_opponent_choose) => match my_choose {
            MyChoices::X(my_choose) => FightResult::DRAW,
            MyChoices::Y(my_choose) => FightResult::WIN,
            MyChoices::Z(my_choose) => FightResult::LOST,
        },
        OpponentChoices::B(opponent_choose) => match my_choose {
            MyChoices::X(my_choose) => FightResult::LOST,
            MyChoices::Y(my_choose) => FightResult::DRAW,
            MyChoices::Z(my_choose) => FightResult::WIN,
        },
        OpponentChoices::C(opponent_choose) => match my_choose {
            MyChoices::X(my_choose) => FightResult::WIN,
            MyChoices::Y(my_choose) => FightResult::LOST,
            MyChoices::Z(my_choose) => FightResult::DRAW,
        },
    };

    let points_from_result = get_points_for_result(fight_result);
    let points_from_choose = get_points_for_my_choose(my_choose);

    return points_from_choose + points_from_result;
}

fn main() {
    let args = common::read_args();
    let input_file = common::read_file(&args[1]);
    let mut total_score: u64 = 0;

    for row in input_file {
        match row {
            Ok(n) => {
                let pair: Vec<&str> = n.split(" ").collect();
                let choices = get_choices(pair);
                let result = fight(&choices.0, &choices.1);

                total_score = total_score + u64::from(result);
            }
            Err(e) => {
                println!("Error reading line: {}", e)
            }
        }
    }
    println!("Total score result is: {}", total_score);
}
