mod solution1;
mod solution2;
fn main() {
    let a: [[i8; 6]; 6] = [
        [0, 0, 0, 0, 0, 0],
        [0, 0, 2, 0, 0, 0],
        [1, 1, 2, 3, 0, 0],
        [0, 0, 0, 3, 0, 0],
        [0, 0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0, 0],
    ];

    let b: [[i8; 6]; 6] = [
        [2, 2, 2, 0, 0, 3],
        [0, 0, 4, 0, 0, 3],
        [1, 1, 4, 0, 0, 3],
        [5, 0, 4, 0, 6, 6],
        [5, 0, 0, 0, 7, 0],
        [8, 8, 8, 0, 7, 0],
    ];

    let solutions = solution2::solve(b);
    println!("solution steps: {:?}", solutions);

    solution2::print_solution(b, &solutions)
}
