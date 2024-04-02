use std::{
    collections::{HashMap, VecDeque},
    fmt::{Display, Formatter, Result},
};

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

#[derive(Clone, PartialEq, Eq, Hash)]
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
        let mut steps: Vec<Step> = Vec::new();
        let mut positions: HashMap<i8, Vec<(usize, usize)>> = HashMap::new();

        for i in 0..6 {
            for j in 0..6 {
                let car_id = self.0[i][j];
                if car_id != 0 {
                    positions.entry(car_id).or_default().push((i, j));
                }
            }
        }

        for (&car_id, positions) in &positions {
            let is_horizontal = positions.iter().all(|&(r, _)| r == positions[0].0);

            if is_horizontal {
                let left_most = positions.iter().min_by_key(|&&(_, c)| c).unwrap().1;
                let right_most = positions.iter().max_by_key(|&&(_, c)| c).unwrap().1;

                if left_most > 0 && self.0[positions[0].0][left_most - 1] == 0 {
                    steps.push(Step {
                        car_id,
                        direction: MovementDirection::Left,
                    });
                }
                if right_most < 5 && self.0[positions[0].0][right_most + 1] == 0 {
                    steps.push(Step {
                        car_id,
                        direction: MovementDirection::Right,
                    });
                }
            } else {
                let top_most = positions.iter().min_by_key(|&&(r, _)| r).unwrap().0;
                let bottom_most = positions.iter().max_by_key(|&&(r, _)| r).unwrap().0;

                if top_most > 0 && self.0[top_most - 1][positions[0].1] == 0 {
                    steps.push(Step {
                        car_id,
                        direction: MovementDirection::Up,
                    });
                }
                if bottom_most < 5 && self.0[bottom_most + 1][positions[0].1] == 0 {
                    steps.push(Step {
                        car_id,
                        direction: MovementDirection::Down,
                    });
                }
            }
        }

        steps
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
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    queue.push_back((initial_board, Vec::new()));

    while let Some((current_board, steps)) = queue.pop_front() {
        if current_board.is_goal() {
            return steps;
        }

        if visited.insert(current_board.clone(), true).is_some() {
            continue;
        }

        for step in current_board.get_possible_steps() {
            let next_board = current_board.apply_step(step.clone());
            if visited.get(&next_board).is_none() {
                let mut next_steps = steps.clone();
                next_steps.push(step.clone());
                queue.push_back((next_board, next_steps));
            }
        }
    }

    Vec::new()
}

pub fn print_solution(board: [[i8; 6]; 6], solution: &[Step]) {
    let mut current_board = Board::new(board);
    println!("Initial Board:");
    current_board.print_board();

    for step in solution {
        println!("Applying step: Move Car {} {}", step.car_id, step.direction);
        current_board = current_board.apply_step(step.clone());
        current_board.print_board();
    }
}
