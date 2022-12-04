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
    ROCK(String),
    PAPER(String),
    SCISSORS(String),
}

enum MyChoices {
    ROCK(String),
    PAPER(String),
    SCISSORS(String),
}

enum ExpectedFightResult {
    LOST(String),
    DRAW(String),
    WIN(String),
}

fn get_response_for_expected_result(
    opponent_choose: &OpponentChoices,
    expected_result: &ExpectedFightResult,
) -> MyChoices {
    return match expected_result {
        ExpectedFightResult::LOST(_) => match opponent_choose {
            OpponentChoices::ROCK(_) => MyChoices::SCISSORS("C".to_string()),
            OpponentChoices::PAPER(_) => MyChoices::ROCK("A".to_string()),
            OpponentChoices::SCISSORS(_) => MyChoices::PAPER("B".to_string()),
        },
        ExpectedFightResult::DRAW(_) => match opponent_choose {
            OpponentChoices::ROCK(_) => MyChoices::ROCK("A".to_string()),
            OpponentChoices::PAPER(_) => MyChoices::PAPER("B".to_string()),
            OpponentChoices::SCISSORS(_) => MyChoices::SCISSORS("C".to_string()),
        },
        ExpectedFightResult::WIN(_) => match opponent_choose {
            OpponentChoices::ROCK(_) => MyChoices::PAPER("B".to_string()),
            OpponentChoices::PAPER(_) => MyChoices::SCISSORS("C".to_string()),
            OpponentChoices::SCISSORS(_) => MyChoices::ROCK("A".to_string()),
        },
    };
}

fn get_points_for_result(result: FightResult) -> u8 {
    return match result {
        FightResult::LOST => 0,
        FightResult::DRAW => 3,
        FightResult::WIN => 6,
    };
}

fn get_points_for_my_choose(my_choose: &MyChoices) -> u8 {
    return match my_choose {
        MyChoices::ROCK(_) => 1,
        MyChoices::PAPER(_) => 2,
        MyChoices::SCISSORS(_) => 3,
    };
}

fn get_opponent_choose(choose: &str) -> OpponentChoices {
    return match choose.as_ref() {
        "A" => OpponentChoices::ROCK(choose.to_string()),
        "B" => OpponentChoices::PAPER(choose.to_string()),
        "C" => OpponentChoices::SCISSORS(choose.to_string()),
        _ => panic!(),
    };
}

fn get_my_choose(choose: &str) -> MyChoices {
    return match choose.as_ref() {
        "X" => MyChoices::ROCK(choose.to_string()),
        "Y" => MyChoices::PAPER(choose.to_string()),
        "Z" => MyChoices::SCISSORS(choose.to_string()),
        _ => panic!(),
    };
}

fn get_expected_result(result: &str) -> ExpectedFightResult {
    return match result.as_ref() {
        "X" => ExpectedFightResult::LOST(result.to_string()),
        "Y" => ExpectedFightResult::DRAW(result.to_string()),
        "Z" => ExpectedFightResult::WIN(result.to_string()),
        _ => panic!(),
    };
}

fn fight(opponent_choose: &OpponentChoices, my_choose: &MyChoices) -> u8 {
    let fight_result: FightResult = match opponent_choose {
        OpponentChoices::ROCK(_) => match my_choose {
            MyChoices::ROCK(_) => FightResult::DRAW,
            MyChoices::PAPER(_) => FightResult::WIN,
            MyChoices::SCISSORS(_) => FightResult::LOST,
        },
        OpponentChoices::PAPER(_) => match my_choose {
            MyChoices::ROCK(_) => FightResult::LOST,
            MyChoices::PAPER(_) => FightResult::DRAW,
            MyChoices::SCISSORS(_) => FightResult::WIN,
        },
        OpponentChoices::SCISSORS(_) => match my_choose {
            MyChoices::ROCK(_) => FightResult::WIN,
            MyChoices::PAPER(_) => FightResult::LOST,
            MyChoices::SCISSORS(_) => FightResult::DRAW,
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

                let opponent_choose = get_opponent_choose(pair[0]);
                let expected_result = get_expected_result(pair[1]);
                let my_choice =
                    get_response_for_expected_result(&opponent_choose, &expected_result);
                let result = fight(&opponent_choose, &my_choice);

                total_score = total_score + u64::from(result);
            }
            Err(e) => {
                println!("Error reading line: {}", e)
            }
        }
    }
    println!("Total score result is: {}", total_score);
}
