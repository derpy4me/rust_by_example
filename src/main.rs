mod codewars;
mod first;
fn main() {
    first::formatted_print::main();

    assert_eq!(codewars::multiples::solution(10), 23);
    assert_eq!(codewars::multiples::solution(11), 33);
    assert_eq!(codewars::multiples::solution(6), 8);
}
