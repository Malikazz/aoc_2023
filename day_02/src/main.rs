use day_02;
pub fn main() {
    println!(
        "{:?}",
        day_02::solve(day_02::read_lines("./day_02/src/input"))
    );
    println!(
        "{:?}",
        day_02::solve_part_two(day_02::read_lines("./day_02/src/input"))
    );
}
