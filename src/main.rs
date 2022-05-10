mod codewars;
mod first;
fn main() {
    first::formatted_print::main();
    test_multiples();
    test_maskify();
    test_trolls();
    test_letter_scores();
    test_array_diff();
    test_valid_braces();
    test_pile_of_cubes();
}

fn test_multiples() {
    assert_eq!(codewars::multiples::my_solution(10), 23);
    assert_eq!(codewars::multiples::my_solution(11), 33);
    assert_eq!(codewars::multiples::my_solution(33), 225);
    assert_eq!(codewars::multiples::clever_solution(10500), 25719750);
    assert_eq!(codewars::multiples::clever_solution(50), 543);
}

fn test_maskify() {
    assert_eq!(
        codewars::maskify::maskify("4556364607935616"),
        "############5616"
    );
    assert_eq!(codewars::maskify::maskify("1"), "1");
    assert_eq!(codewars::maskify::maskify("11111"), "#1111");
    assert_eq!(
        codewars::maskify::clever_maskify("4556364607935616"),
        "############5616"
    );
    assert_eq!(codewars::maskify::clever_maskify("1"), "1");
    assert_eq!(codewars::maskify::clever_maskify("11111"), "#1111");
}

fn test_trolls() {
    assert_eq!(
        codewars::trolls::disemvowels("This has vowels"),
        "Ths hs vwls"
    );
    assert_eq!(
        codewars::trolls::disemvowels("This website is for losers LOL!"),
        "Ths wbst s fr lsrs LL!"
    );
    assert_eq!(
        codewars::trolls::how_i_originally_wanted_to_do_it("This has vowels"),
        "Ths hs vwls"
    );
    assert_eq!(
        codewars::trolls::how_i_originally_wanted_to_do_it("This website is for losers LOL!"),
        "Ths wbst s fr lsrs LL!"
    );
}

fn test_letter_scores() {
    assert_eq!(
        codewars::letter_score::printer_error(
            "aaaaaaaaaaaaaaaaabbbbbbbbbbbbbbbbbbbbbbmmmmmmmmmmmmxyz"
        ),
        "3/54"
    )
}

fn test_array_diff() {
    assert_eq!(
        codewars::array_diff::array_diff(vec![1, 2], vec![1]),
        vec![2]
    );
    assert_eq!(
        codewars::array_diff::array_diff(vec![1, 2, 2], vec![1]),
        vec![2, 2]
    );
    assert_eq!(
        codewars::array_diff::array_diff(vec![1, 2, 2], vec![2]),
        vec![1]
    );
    assert_eq!(
        codewars::array_diff::array_diff(vec![1, 2, 2], vec![]),
        vec![1, 2, 2]
    );
    assert_eq!(codewars::array_diff::array_diff(vec![], vec![1, 2]), vec![]);
    assert_eq!(
        codewars::array_diff::array_diff(vec![1, 2, 3], vec![1, 2]),
        vec![3]
    );
}

fn test_valid_braces() {
    let expect_true = "()";
    let expect_false = "[(])";
    assert!(
        codewars::valid_braces::valid_braces(expect_true),
        "Expected {s:?} to be valid. Got false",
        s = expect_true
    );
    assert!(
        !codewars::valid_braces::valid_braces(expect_false),
        "Expected {s:?} to be invalid. Got true",
        s = expect_false
    );
    assert!(
        codewars::valid_braces::match_valid_braces(expect_true),
        "Expected {s:?} to be valid. Got false",
        s = expect_true
    );
    assert!(
        !codewars::valid_braces::match_valid_braces(expect_false),
        "Expected {s:?} to be invalid. Got true",
        s = expect_false
    );
    assert!(
        codewars::valid_braces::clever_valid_braces(expect_true),
        "Expected {s:?} to be valid. Got false",
        s = expect_true
    );
    assert!(
        !codewars::valid_braces::clever_valid_braces(expect_false),
        "Expected {s:?} to be invalid. Got true",
        s = expect_false
    );
}

fn test_pile_of_cubes() {
    fn testing(n: u64, exp: i32) -> () {
        assert_eq!(codewars::pile_of_cubes::find_nb(n), exp);
        assert_eq!(codewars::pile_of_cubes::best_find_nb(n), exp);
    }

    testing(4183059834009, 2022);
    testing(24723578342962, -1);
    testing(135440716410000, 4824);
    testing(40539911473216, 3568);
    testing(26825883955641, 3218);
}
