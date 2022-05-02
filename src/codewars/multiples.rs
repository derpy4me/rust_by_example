pub fn my_solution(num: i32) -> i32 {
    let mut total = 0;
    for number in 0..num {
        if number % 5 == 0 {
            total += number;
        } else if number % 3 == 0 {
            total += number;
        }
    }
    println!("{total}");
    return total;
}

pub fn clever_solution(num: i32) -> i32 {
    (1..num).filter(|x| x % 3 == 0 || x % 5 == 0).sum()
}
