mod codewars;
mod first;
fn main() {
    first::formatted_print::main();
    test_multiples();
    test_maskify();
    test_trolls();
    test_letter_scores();
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
