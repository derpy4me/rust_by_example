pub fn solution(num: i32) -> i32 {
    let mut num_list = Vec::new();
    for number in 0..num {
        if number % 5 == 0 && number % 3 == 0 {
            num_list.push(number);
        } else if number % 5 == 0 {
            num_list.push(number);
        } else if number % 3 == 0 {
            num_list.push(number);
        }
    }

    let mut total = 0;

    for another in num_list.iter() {
        total += another;
    }

    return total;
}
