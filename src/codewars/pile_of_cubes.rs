pub fn find_nb(m: u64) -> i32 {
    let mut m = m;
    let mut total: u64 = 1;
    while m > 0 {
        let exp = total.pow(3);
        if m >= exp {
            m -= exp;
            total += 1;
        } else {
            return -1;
        }
    }
    return (total - 1) as i32;
}

pub fn best_find_nb(n: u64) -> i32 {
    let mut sum = 0_u64;
    let l = (0_u64..)
        .take_while(|&x| {
            sum += x.pow(3);
            sum < n
        })
        .count() as i32;
    if sum == n {
        l
    } else {
        -1
    }
}
