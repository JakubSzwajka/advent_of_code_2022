use std::usize;

pub mod common;

const PLANE_SIZE_X: usize = 30;
const PLANE_SIZE_Y: usize = 30;

const STARTING_X: i32 = 0;
const STARTING_Y: i32 = 0;

#[derive(Debug)]
struct MoveInfo {
    // not using move since it is reserved keyword
    direction: char,
    how_far: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    name: char,
    x: i32,
    y: i32,
    prev_x: i32,
    prev_y: i32,
}

impl Point {
    fn new(name: char, x: i32, y: i32) -> Point {
        Point {
            name: name,
            x: x,
            y: y,
            prev_x: x,
            prev_y: y,
        }
    }

    fn get_diagonal_neighbours(&self) -> [Point; 4] {
        return [
            Point::new('N', self.x - 1, self.y),
            Point::new('N', self.x + 1, self.y),
            Point::new('N', self.x, self.y - 1),
            Point::new('N', self.x, self.y + 1),
        ];
    }
}

#[derive(Debug)]
struct Snake {
    head: Point,
    tail: Vec<Point>,
    tail_history: Vec<Point>,
}

impl Snake {
    fn new(length: u32, head_pos: (i32, i32)) -> Snake {
        let mut tail: Vec<Point> = Vec::new();

        for (_i, e) in (1..length).enumerate() {
            // in length includes head  ( -1 to have tail lenght)
            tail.push(Point::new(
                e.to_string().parse().unwrap(),
                head_pos.0,
                head_pos.1,
            ));
        }

        let mut tail_history: Vec<Point> = Vec::new();
        tail_history.push(tail[tail.len() - 1]);

        return Snake {
            head: Point::new('H', head_pos.0, head_pos.1),
            tail: tail,
            tail_history: tail_history,
        };
    }

    fn do_move(&mut self, move_info: &MoveInfo) -> () {
        for _i in 0..move_info.how_far {
            println!("----------- Moving {} ------------", move_info.direction);
            match move_info.direction {
                'R' => {
                    self.head.prev_x = self.head.x;
                    self.head.prev_y = self.head.y;
                    self.head.x = self.head.x + 1;
                }
                'L' => {
                    self.head.prev_x = self.head.x;
                    self.head.prev_y = self.head.y;
                    self.head.x = self.head.x - 1;
                }
                'U' => {
                    self.head.prev_x = self.head.x;
                    self.head.prev_y = self.head.y;
                    self.head.y = self.head.y - 1;
                }
                'D' => {
                    self.head.prev_x = self.head.x;
                    self.head.prev_y = self.head.y;
                    self.head.y = self.head.y + 1;
                }
                _ => {
                    panic!("Direction {} not known", move_info.direction)
                }
            }

            let mut last_moved_node: &Point = &self.head;

            for moving_node in &mut self.tail {
                let dist = calculate_distance(&moving_node, &last_moved_node);
                if dist == (2 as f32).sqrt() * 2.0 {
                    // actually there is one more type of move. Across the nodes.
                    // Distance is always eq 2 * sqrt(2) and we move to the prev pos of
                    // the next node
                    moving_node.prev_x = moving_node.x;
                    moving_node.prev_y = moving_node.y;
                    moving_node.x = last_moved_node.prev_x;
                    moving_node.y = last_moved_node.prev_y;
                } else if dist >= 2.0 {
                    let new_destination_point = get_move(moving_node, last_moved_node);
                    moving_node.prev_x = moving_node.x;
                    moving_node.prev_y = moving_node.y;
                    moving_node.x = new_destination_point.x;
                    moving_node.y = new_destination_point.y;
                }
                last_moved_node = moving_node;
            }

            self.tail_history.push(Point::new(
                'X',
                self.tail[self.tail.len() - 1].x,
                self.tail[self.tail.len() - 1].y,
            ));
            // self.print();
        }
    }

    fn print(&self) -> () {
        let mut plane: [[char; PLANE_SIZE_X]; PLANE_SIZE_Y] = [['.'; PLANE_SIZE_X]; PLANE_SIZE_Y];

        plane[self.head.y as usize][self.head.x as usize] = self.head.name;
        for node in &self.tail {
            if plane[node.y as usize][node.x as usize] == '.' {
                plane[node.y as usize][node.x as usize] = node.name;
            }
        }

        for row in plane {
            println!("{:?}", row);
        }
    }
}

fn calculate_distance(a: &Point, b: &Point) -> f32 {
    let x = ((a.x - b.x).pow(2) + (a.y - b.y).pow(2)) as f32;
    return x.sqrt();
}

fn get_move(to_move: &Point, moved: &Point) -> Point {
    let mut possible_moves = moved
        .get_diagonal_neighbours()
        .map(|x| (x, calculate_distance(to_move, &x)));

    possible_moves.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    return possible_moves[0].0;
}

fn main() {
    let args = common::read_args();
    let data = common::read_file(&args[1]).unwrap();

    let moves = data
        .lines()
        .map(|c| {
            let chars = c.split(" ").collect::<Vec<&str>>();
            return MoveInfo {
                direction: chars[0].chars().collect::<Vec<char>>()[0],
                how_far: chars[1].parse().unwrap(),
            };
        })
        .collect::<Vec<MoveInfo>>();

    let mut snake = Snake::new(10, (STARTING_X, STARTING_Y));

    for m in moves {
        snake.do_move(&m);
        // snake.print();
    }
    snake.tail_history.sort();
    snake.tail_history.dedup_by(|a, b| a.x == b.x && a.y == b.y);

    dbg!(&snake.tail_history.len());
}
