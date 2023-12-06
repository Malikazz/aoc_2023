use day_02;
use aoc_file_parser::read_lines;
pub fn main() {
    println!(
        "{:?}",
        day_02::solve(read_lines("./day_02/src/input"))
    );
    println!(
        "{:?}",
        day_02::solve_part_two(read_lines("./day_02/src/input"))
    );
}
