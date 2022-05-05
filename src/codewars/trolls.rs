pub fn disemvowels(troll: &str) -> String {
    troll.replace(&['a', 'A', 'e', 'E', 'i', 'I', 'o', 'O', 'u', 'U'][..], "")
}

pub fn how_i_originally_wanted_to_do_it(troll: &str) -> String {
    troll
        .chars()
        .filter(|&c| !"aeiou".contains(c.to_ascii_lowercase()))
        .collect()
}
