pub mod common;
pub use transpose::transpose;

#[derive(Debug)]
struct Stack {
    index: u32,
    crates: Vec<char>,
}

#[derive(Debug)]
struct Command {
    quant: u32,
    from: u32,
    to: u32,
}

impl Command {
    fn to_str(&self) -> String {
        return format!("Move {} from {} to {}", self.quant, self.from, self.to);
    }
}

#[derive(Debug)]
struct CargoCrane {
    stacks: Vec<Stack>,
}

impl CargoCrane {
    fn new() -> CargoCrane {
        return CargoCrane { stacks: Vec::new() };
    }

    fn apply_comamnd(&mut self, command: &Command) {
        let mut create_handler: Vec<char> = Vec::new();

        for _ in 0..command.quant {
            let x = self.stacks[(command.from - 1) as usize].crates.remove(0);
            create_handler.push(x);
        }
        for _ in 0..create_handler.len() {
            let y = create_handler.remove(create_handler.len() - 1);
            self.stacks[(command.to - 1) as usize].crates.insert(0, y);
        }
    }
}

fn parse_commands(rearrangement_procedure: &Vec<&str>) -> Vec<Command> {
    let mut commands: Vec<Command> = Vec::new();

    for p in rearrangement_procedure {
        let data = p.split(" ").collect::<Vec<&str>>();
        commands.push(Command {
            quant: data[1].parse().unwrap(),
            from: data[3].parse().unwrap(),
            to: data[5].parse().unwrap(),
        })
    }
    return commands;
}

fn parse_cargo_crane(stacks_data: &Vec<&str>) -> CargoCrane {
    let mut cargo_crane: CargoCrane = CargoCrane::new();

    for (i, element) in stacks_data[stacks_data.len() - 1].chars().enumerate() {
        if element.is_numeric() {
            let mut stack = Stack {
                index: element.to_digit(10).unwrap(),
                crates: Vec::new(),
            };

            for line in stacks_data {
                let x = line.chars().collect::<Vec<char>>();
                if x[i].is_alphabetic() {
                    stack.crates.push(x[i]);
                }
            }
            cargo_crane.stacks.push(stack);
        }
    }
    return cargo_crane;
}

fn main() {
    let args = common::read_args();
    let starting_cargo_data = common::read_file(&args[1]).unwrap();
    let rearrangement_procedure_data = common::read_file(&args[2]).unwrap();

    let cargo_data = starting_cargo_data.lines().collect::<Vec<_>>();
    let rearrangement_procedure = rearrangement_procedure_data.lines().collect::<Vec<_>>();

    let commands: Vec<Command> = parse_commands(&rearrangement_procedure);
    let mut cargo = parse_cargo_crane(&cargo_data);

    dbg!(&commands);

    for command in commands {
        cargo.apply_comamnd(&command);
    }

    let result = cargo
        .stacks
        .iter()
        .map(|x| x.crates.first().unwrap())
        .collect::<Vec<&char>>();

    dbg!(&result);
}
