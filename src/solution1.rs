use std::{
    collections::HashSet,
    error::Error,
    hash::{Hash, Hasher},
};

#[derive(PartialEq, Eq, Clone, Debug)]
enum MovementDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Step {
    car_id: i8,
    direction: MovementDirection,
}

enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Eq, Clone)]
struct State {
    board: [[i8; 6]; 6],
    total_cost: u32,
    steps: Vec<Step>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for row in self.board.iter() {
            for &cell in row.iter() {
                cell.hash(state)
            }
        }
    }
}

impl State {
    fn get_orientation(&self, car_id: i8) -> Option<Orientation> {
        for x in 0..6 {
            for y in 0..6 {
                if self.board[x][y] == car_id {
                    // Check if the next horizontal grid has the same id
                    if y < 5 && self.board[x][y + 1] == car_id {
                        return Some(Orientation::Horizontal);
                    }
                    // Check if the next vertical grid has the same id
                    if x < 5 && self.board[x + 1][y] == car_id {
                        return Some(Orientation::Vertical);
                    }
                }
            }
        }
        None
    }

    fn can_move(&self, car_id: i8, direction: &MovementDirection) -> bool {
        match self.get_orientation(car_id) {
            Some(Orientation::Vertical) => match direction {
                MovementDirection::Up | MovementDirection::Down => {
                    self.can_move_vertical(car_id, direction)
                }
                _ => false,
            },
            Some(Orientation::Horizontal) => match direction {
                MovementDirection::Left | MovementDirection::Right => {
                    self.can_move_horizontal(car_id, direction)
                }
                _ => false,
            },
            _ => false,
        }
    }

    fn can_move_horizontal(&self, car_id: i8, direction: &MovementDirection) -> bool {
        for x in 0..6 {
            for y in 0..6 {
                if self.board[x][y] == car_id {
                    match direction {
                        MovementDirection::Left => {
                            if y == 0
                                || (self.board[x][y - 1] != 0 && self.board[x][y - 1] != car_id)
                            {
                                return false;
                            }
                        }
                        MovementDirection::Right => {
                            if y == 5
                                || (self.board[x][y + 1] != 0 && self.board[x][y + 1] != car_id)
                            {
                                return false;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        true
    }

    fn can_move_vertical(&self, car_id: i8, direction: &MovementDirection) -> bool {
        for x in 0..6 {
            for y in 0..6 {
                if self.board[x][y] == car_id {
                    match direction {
                        MovementDirection::Up => {
                            if x == 0
                                || (self.board[x - 1][y] != 0 && self.board[x - 1][y] != car_id)
                            {
                                return false;
                            }
                        }
                        MovementDirection::Down => {
                            if x == 5
                                || (self.board[x + 1][y] != 0 && self.board[x + 1][y] != car_id)
                            {
                                return false;
                            }
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        true
    }
}

fn generate_successors(state: &State) -> Vec<State> {
    let mut successors = Vec::<State>::new();

    for i in 0..6 {
        for j in 0..6 {
            let car_id = state.board[i][j];
            // 0 representing empty space, if it is not 0, then this is some car
            if car_id != 0 {
                for direction in &[
                    MovementDirection::Up,
                    MovementDirection::Down,
                    MovementDirection::Left,
                    MovementDirection::Right,
                ] {
                    if state.can_move(car_id, direction) {
                        let mut new_board = state.board;
                        move_car(car_id, direction, &mut new_board);

                        let mut new_steps = state.steps.clone();
                        new_steps.push(Step {
                            car_id,
                            direction: direction.clone(),
                        });

                        successors.push(State {
                            board: new_board,
                            total_cost: state.total_cost + 1,
                            steps: new_steps,
                        })
                    }
                }
            }
        }
    }
    successors
}

fn move_car(car_id: i8, direction: &MovementDirection, board: &mut [[i8; 6]; 6]) {
    let mut car_pos = Vec::new();

    for (x, row) in board.iter().enumerate() {
        for (y, col) in row.iter().enumerate() {
            if *col == car_id {
                car_pos.push((x, y));
            }
        }
    }

    // Sort to make sure the start of the car_pos vec has the correct head
    match direction {
        MovementDirection::Up | MovementDirection::Left => car_pos.sort(),
        MovementDirection::Down | MovementDirection::Right => car_pos.sort_by(|a, b| b.cmp(a)),
    }

    // Update the board by moving the car
    for pos in car_pos {
        let (x, y) = pos;
        board[x][y] = 0;

        match direction {
            MovementDirection::Up => board[x - 1][y] = car_id,
            MovementDirection::Down => board[x + 1][y] = car_id,
            MovementDirection::Left => board[x][y - 1] = car_id,
            MovementDirection::Right => board[x][y + 1] = car_id,
        }
    }
}

fn heuristic(state: &State) -> u32 {
    // Distance between red car head to exit
    6 - state.board[2].iter().rposition(|x| *x == 1).unwrap() as u32 - 1
}

fn search(
    state: &State,
    total_cost: u32,
    threshold: u32,
    visited: &mut HashSet<State>,
) -> (bool, u32, Vec<Step>) {
    if visited.contains(state) {
        return (false, std::u32::MAX, Vec::new());
    }

    let cost = total_cost + heuristic(state);

    if cost > threshold {
        return (false, cost, Vec::new());
    }

    if state.board[2][4] == 1 && state.board[2][5] == 1 {
        return (true, threshold, state.steps.clone());
    }

    visited.insert(state.clone());

    let mut min = std::u32::MAX;
    for successor in generate_successors(state) {
        let (found, new_threshold, steps) = search(&successor, total_cost + 1, threshold, visited);
        if found {
            return (true, new_threshold, steps);
        } else if new_threshold > threshold && new_threshold < min {
            min = new_threshold
        }
    }
    visited.remove(state);

    (false, min, Vec::<Step>::new())
}

pub fn solve(board: [[i8; 6]; 6]) -> Result<Vec<Step>, Box<dyn Error>> {
    let state = State {
        total_cost: 0,
        board,
        steps: Vec::<Step>::new(),
    };

    let mut threshold = heuristic(&state);
    let mut visited = HashSet::<State>::new();

    loop {
        let (found, new_threshold, steps) =
            search(&state, state.total_cost, threshold, &mut visited);
        if found {
            return Ok(steps);
        } else if new_threshold == std::u32::MAX {
            return Ok(Vec::<Step>::new());
        }

        threshold = new_threshold;
        println!("{}", threshold);
    }
}
