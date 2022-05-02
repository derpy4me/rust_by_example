mod codewars;
mod first;
fn main() {
    first::formatted_print::main();

    assert_eq!(codewars::multiples::my_solution(10), 23);
    assert_eq!(codewars::multiples::my_solution(11), 33);
    assert_eq!(codewars::multiples::my_solution(33), 225);
    assert_eq!(codewars::multiples::clever_solution(10500), 25719750);
    assert_eq!(codewars::multiples::clever_solution(50), 543);
}
