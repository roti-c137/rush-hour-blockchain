#![allow(unused)]
use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter, Result},
    mem::size_of_val,
    time::Instant,
};
const MAX_ARR_INDEX: usize = 5;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum MovementDirection {
    Up,
    Right,
    Down,
    Left,
}

impl Display for MovementDirection {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                MovementDirection::Up => "Up",
                MovementDirection::Right => "Right",
                MovementDirection::Down => "Down",
                MovementDirection::Left => "Left",
            }
        )
    }
}

#[derive(Debug, Clone)]
pub struct Step {
    car_id: i8,
    direction: MovementDirection,
}

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Board([[i8; 6]; 6]);

impl Board {
    fn new(board: [[i8; 6]; 6]) -> Self {
        Self(board)
    }

    fn print_board(&self) {
        println!("Current board state:");
        for row in self.0.iter() {
            for &cell in row.iter() {
                print!("{:2} ", cell);
            }
            println!();
        }
        println!("---------------------------------");
    }

    fn is_goal(&self) -> bool {
        // The goal is achieved when the red car (1) is right before the exit on the third row
        self.0[2][4] == 1 && self.0[2][5] == 1
    }

    fn get_possible_steps(&self) -> Vec<Step> {
        let mut car_searched: HashMap<i8, bool> = HashMap::new();
        let mut steps_new: Vec<Step> = Vec::new();
        let mut iteration1 = 0;
        {
            for x in 0..6 {
                for y in 0..6 {
                    let car_id = self.0[x][y];
                    if car_id == 0 || car_searched.get(&car_id).is_some() {
                        continue;
                    }
                    car_searched.insert(car_id, true);

                    // Search car surrounding, from 0.0
                    // Search right
                    let right_coor = y + 1;
                    if (right_coor <= MAX_ARR_INDEX && self.0[x][right_coor] == car_id) {
                        // can car move left?
                        if y > 0 && self.0[x][y - 1] == 0 {
                            steps_new.push(Step {
                                car_id,
                                direction: MovementDirection::Left,
                            })
                        }

                        // can car move right?
                        let mut move_right_coor = right_coor + 1;
                        // 3 coordinate car
                        if move_right_coor <= MAX_ARR_INDEX && self.0[x][move_right_coor] == car_id
                        {
                            move_right_coor += 1;
                        }

                        if move_right_coor <= MAX_ARR_INDEX && self.0[x][move_right_coor] == 0 {
                            steps_new.push(Step {
                                car_id,
                                direction: MovementDirection::Right,
                            })
                        }
                    }

                    // Search down
                    let down_coor = x + 1;
                    if down_coor <= MAX_ARR_INDEX && self.0[down_coor][y] == car_id {
                        // can car move up?
                        if x > 0 && self.0[x - 1][y] == 0 {
                            steps_new.push(Step {
                                car_id,
                                direction: MovementDirection::Up,
                            })
                        }

                        // can car move down?
                        let mut move_down_coor = down_coor + 1;
                        // 3 coord car
                        if move_down_coor <= MAX_ARR_INDEX && self.0[move_down_coor][y] == car_id {
                            move_down_coor += 1;
                        }

                        if move_down_coor <= MAX_ARR_INDEX && self.0[move_down_coor][y] == 0 {
                            steps_new.push(Step {
                                car_id,
                                direction: MovementDirection::Down,
                            })
                        }
                    }
                    iteration1 += 1;
                }
            }
        }

        steps_new
    }

    fn apply_step(&self, step: Step) -> Self {
        let mut new_board = self.clone();
        let (dx, dy) = match step.direction {
            MovementDirection::Up => (-1, 0),
            MovementDirection::Right => (0, 1),
            MovementDirection::Down => (1, 0),
            MovementDirection::Left => (0, -1),
        };

        // Clear the car's current position
        for i in 0..6 {
            for j in 0..6 {
                if new_board.0[i][j] == step.car_id {
                    new_board.0[i][j] = 0;
                }
            }
        }

        // Set the car's new position
        for i in 0..6 {
            for j in 0..6 {
                if self.0[i][j] == step.car_id {
                    let new_i = (i as i32 + dx) as usize;
                    let new_j = (j as i32 + dy) as usize;
                    new_board.0[new_i][new_j] = step.car_id;
                }
            }
        }

        new_board
    }
}

pub fn solve(board: [[i8; 6]; 6]) -> Vec<Step> {
    let initial_board = Board::new(board);
    // println!(
    //     "The size of initial_board is {}",
    //     size_of_val(&initial_board)
    // );
    // let compact_initial_board = "2220030040031140003504066500070888070";
    // println!(
    //     "The size of compact_initial_board is {}",
    //     size_of_val(&compact_initial_board)
    // );

    let mut visited_board = HashMap::new();
    let mut queue_board = VecDeque::new();
    queue_board.push_front((initial_board, Vec::new()));

    let mut count = 1;
    let now1 = Instant::now();
    while let Some((current_board, historical_steps)) = queue_board.pop_front() {
        // println!("Queue of board with steps:{:#?}", queue_board);
        if current_board.is_goal() {
            println!("Finish\nTotal iterations: {:#?}", count); // 4544
            println!("Size of visited_board: {}", visited_board.keys().len());
            println!("Elapsed: {:.2?}", now1.elapsed());
            return historical_steps;
        }

        if visited_board.insert(current_board.clone(), true).is_some() {
            continue;
        }

        let steps = current_board.get_possible_steps();
        for step in steps {
            let next_board = current_board.apply_step(step.clone());
            count += 1;
            if visited_board.get(&next_board).is_none() {
                let mut new_historical_steps = historical_steps.clone();
                new_historical_steps.push(step.clone());
                queue_board.push_back((next_board, new_historical_steps));
            }
        }
    }

    Vec::new()
}

pub fn print_solution(board: [[i8; 6]; 6], solution: &[Step]) {
    let mut current_board = Board::new(board);
    println!("Initial Board:");

    current_board.print_board();

    // for step in solution {
    //     println!("Applying step: Move Car {} {}", step.car_id, step.direction);
    //     current_board = current_board.apply_step(step.clone());
    //     current_board.print_board();
    // }
}
