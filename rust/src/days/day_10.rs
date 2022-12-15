pub mod common;

struct CRT {
    screen: [char; 40 * 6],
    sprite_pos: i32,
    cycle: i32,
}

impl CRT {
    fn new() -> CRT {
        CRT {
            screen: ['_'; 40 * 6],
            sprite_pos: 1,
            cycle: 1,
        }
    }

    fn draw(&mut self) -> () {
        if self.cycle - 1 == self.sprite_pos() - 1
            || self.cycle - 1 == self.sprite_pos()
            || self.cycle - 1 == self.sprite_pos() + 1
        {
            self.screen[(self.cycle as usize) - 1] = '#';
        }
    }

    fn sprite_pos(&self) -> i32 {
        let row = (self.cycle / 40) as u32;
        return self.sprite_pos + (row * 40) as i32;
    }

    fn apply(&mut self, command: &Command) -> () {
        match command.command.as_str() {
            "noop" => {
                self.draw();
                self.cycle = self.cycle + 1;
            }
            "addx" => {
                self.draw();
                self.cycle = self.cycle + 1;
                self.draw();
                self.cycle = self.cycle + 1;
                self.sprite_pos = self.sprite_pos + command.args.unwrap();
            }
            _ => {
                panic!("Operation unknown")
            }
        }
    }

    fn print(&self) -> () {
        println!("{:?}", self.screen[..40].iter().collect::<String>());
        println!("{:?}", self.screen[40..80].iter().collect::<String>());
        println!("{:?}", self.screen[80..120].iter().collect::<String>());
        println!("{:?}", self.screen[120..160].iter().collect::<String>());
        println!("{:?}", self.screen[160..200].iter().collect::<String>());
        println!("{:?}", self.screen[200..].iter().collect::<String>());
    }
}

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

fn main() {
    let args = common::read_args();
    let data = common::read_file(&args[1]).unwrap();

    let commands = data
        .lines()
        .map(|x| Command::new(x))
        .collect::<Vec<Command>>();

    // cycle and value list
    let mut register: Vec<(u32, i32)> = Vec::new();
    register.push((1, 1));

    let mut crt = CRT::new();

    for command in &commands.iter().collect::<Vec<&Command>>() {
        command.apply(&mut register);
        crt.apply(command);
    }

    crt.print();
}
