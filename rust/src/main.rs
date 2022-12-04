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

enum MyChoices {
    X(String),
    Y(String),
    Z(String),
}

enum ExpectedFightResult {
    X(String),
    Y(String),
    Z(String),
}

// fn get_response_for_expected_result(
//     opponent_choose: &OpponentChoices,
//     expected_result: &ExpectedFightResult,
// ) -> MyChoices {
//     return expected_result {
//         ExpectedFightResult::
//     };
// }

fn get_points_for_result(result: FightResult) -> u8 {
    return match result {
        FightResult::LOST => 0,
        FightResult::DRAW => 3,
        FightResult::WIN => 6,
    };
}

fn get_points_for_my_choose(my_choose: &MyChoices) -> u8 {
    return match my_choose {
        MyChoices::X(_) => 1,
        MyChoices::Y(_) => 2,
        MyChoices::Z(_) => 3,
    };
}

fn get_opponent_choose(choose: &str) -> OpponentChoices {
    return match choose.as_ref() {
        "A" => OpponentChoices::A(choose.to_string()),
        "B" => OpponentChoices::B(choose.to_string()),
        "C" => OpponentChoices::C(choose.to_string()),
        _ => panic!(),
    };
}

fn get_my_choose(choose: &str) -> MyChoices {
    return match choose.as_ref() {
        "X" => MyChoices::X(choose.to_string()),
        "Y" => MyChoices::Y(choose.to_string()),
        "Z" => MyChoices::Z(choose.to_string()),
        _ => panic!(),
    };
}

fn fight(opponent_choose: &OpponentChoices, my_choose: &MyChoices) -> u8 {
    let fight_result: FightResult = match opponent_choose {
        OpponentChoices::A(_) => match my_choose {
            MyChoices::X(_) => FightResult::DRAW,
            MyChoices::Y(_) => FightResult::WIN,
            MyChoices::Z(_) => FightResult::LOST,
        },
        OpponentChoices::B(_) => match my_choose {
            MyChoices::X(_) => FightResult::LOST,
            MyChoices::Y(_) => FightResult::DRAW,
            MyChoices::Z(_) => FightResult::WIN,
        },
        OpponentChoices::C(_) => match my_choose {
            MyChoices::X(_) => FightResult::WIN,
            MyChoices::Y(_) => FightResult::LOST,
            MyChoices::Z(_) => FightResult::DRAW,
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

                let result = fight(&get_opponent_choose(pair[0]), &get_my_choose(pair[1]));

                total_score = total_score + u64::from(result);
            }
            Err(e) => {
                println!("Error reading line: {}", e)
            }
        }
    }
    println!("Total score result is: {}", total_score);
}
