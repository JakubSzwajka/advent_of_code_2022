pub mod common;

#[derive(Debug)]
struct Command {
    command: String,
    args: Option<i32>,
}

impl Command {
    fn new(data: &str) -> Command {
        let payload = data.split(" ").collect::<Vec<&str>>();

        if data.contains("noop") {
            return Command::new_noop(payload);
        } else if data.contains("addx") {
            return Command::new_addx(payload);
        } else {
            panic!("Operation unknown")
        }
    }
    fn new_noop(data: Vec<&str>) -> Command {
        Command {
            command: data[0].to_string(),
            args: None,
        }
    }

    fn new_addx(data: Vec<&str>) -> Command {
        Command {
            command: data[0].to_string(),
            args: Some(data[1].parse().unwrap()),
        }
    }

    fn apply(&self, register: &mut Vec<(u32, i32)>) {
        let mut last_op = register[register.len() - 1];
        match self.command.as_str() {
            "noop" => {
                register.push((last_op.0 + 1, last_op.1));
            }
            "addx" => {
                register.push((last_op.0 + 1, last_op.1));
                last_op = register[register.len() - 1];
                register.push((last_op.0 + 1, last_op.1 + self.args.unwrap()));
            }
            _ => {
                panic!("Operation unknown")
            }
        }
    }
}

fn print_register(register: &Vec<(u32, i32)>) -> () {
    let mut last_op: &(u32, i32) = &(0, 0);

    let mut stengths_sum = 0;

    for (i, cycle) in register.iter().enumerate() {
        if i == 20 || ((i > 20) && ((i - 20) % 40) == 0) {
            let signal_strength = i as i32 * last_op.1;
            println!(
                "Register after cycle: {} = {}. Signal strength: {}",
                i, cycle.1, signal_strength
            );
            stengths_sum = stengths_sum + signal_strength;
        }
        last_op = cycle;
    }

    println!("Signal strength sum: {}", stengths_sum);
}

fn main() {
    let args = common::read_args();
    let data = common::read_file(&args[1]).unwrap();

    let commands = data
        .lines()
        .map(|x| Command::new(x))
        .collect::<Vec<Command>>();

    // cycle and value list
    let mut register: Vec<(u32, i32)> = Vec::new();
    register.push((0, 1));

    for command in &commands {
        command.apply(&mut register);
    }
    print_register(&register);
}
