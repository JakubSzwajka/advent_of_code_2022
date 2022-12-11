pub mod common;

#[derive(Debug)]
struct MoveInfo {
    // not using move since it is reserved keyword
    direction: char,
    how_far: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Snake {
    head: Point,
    tail: Point,
    length: u32,
    tail_history: Vec<Point>,
}

impl Snake {
    fn do_move(&mut self, move_info: &MoveInfo) -> () {
        for i in 0..move_info.how_far {
            let head_pos = self.head;

            match move_info.direction {
                'R' => {
                    self.head.x = self.head.x + 1;
                }
                'L' => {
                    self.head.x = self.head.x - 1;
                }
                'U' => {
                    self.head.y = self.head.y + 1;
                }
                'D' => {
                    self.head.y = self.head.y - 1;
                }
                _ => {
                    panic!("Direction {} not known", move_info.direction)
                }
            }
            if calculate_distance(&self.tail, &self.head) >= 2.0 {
                println!("Head at: ({},{})", self.head.x, self.head.y);
                println!(
                    "Moving tails from ({},{}) to ({},{})",
                    self.tail.x, self.tail.y, head_pos.x, head_pos.y
                );
                self.tail = head_pos;
                self.tail_history.push(self.tail.clone());
            }
        }
    }
}

fn calculate_distance(a: &Point, b: &Point) -> f32 {
    let x = ((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f32;
    return x.sqrt();
}

fn main() {
    let args = common::read_args();
    let data = common::read_file(&args[1]).unwrap();

    let moves = data
        .lines()
        .map(|c| {
            let chars = c.split(" ").collect::<Vec<&str>>();
            // let chars = c.chars().collect::<Vec<char>>();
            // let how_far = chars[2..] as String;
            return MoveInfo {
                direction: chars[0].chars().collect::<Vec<char>>()[0],
                how_far: chars[1].parse().unwrap(),
            };
        })
        .collect::<Vec<MoveInfo>>();

    let mut snake = Snake {
        head: Point { x: 0, y: 0 },
        tail: Point { x: 0, y: 0 },
        length: 1,
        tail_history: Vec::new(),
    };

    snake.tail_history.push(Point { x: 0, y: 0 });

    // dbg!(&moves);
    // dbg!(&snake);
    for m in moves {
        snake.do_move(&m);
    }

    snake.tail_history.sort();
    snake.tail_history.dedup_by(|a, b| a.x == b.x && a.y == b.y);

    println!("-------------------------");

    dbg!(&snake.tail_history.len()); // + 1 for starting position
                                     // dbg!(&snake.tail_history);
}
