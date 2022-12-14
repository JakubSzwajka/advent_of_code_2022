pub mod common;

fn main() {
    let args = common::read_args();
    let input_file = common::read_file(&args[1]);

    let mut current_elve_calories: u64 = 0;

    let mut top_elves: Vec<u64> = vec![0, 0, 0];

    for row in input_file {
        match row {
            Ok(n) => match n.parse::<u64>() {
                Ok(n) => {
                    current_elve_calories = current_elve_calories + n;
                }
                Err(_e) => {
                    let lowest_from_top_elves = common::get_vector_info(&top_elves);

                    if current_elve_calories > lowest_from_top_elves.min.0 {
                        top_elves[lowest_from_top_elves.min.1] = current_elve_calories;
                    }
                    current_elve_calories = 0;
                }
            },
            Err(e) => {
                println!("Error while reading the file line: {}", e)
            }
        }
    }

    let lowest_from_top_elves = common::get_vector_info(&top_elves);

    if current_elve_calories > lowest_from_top_elves.min.0 {
        top_elves[lowest_from_top_elves.min.1] = current_elve_calories;
    }

    println!("{:?}", lowest_from_top_elves);
}
