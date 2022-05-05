pub fn maskify(cc: &str) -> String {
    if cc.chars().count() < 5 {
        return cc.to_string();
    }
    let mut hashed = String::new();
    for (i, c) in cc.chars().enumerate() {
        if i > (cc.chars().count() - 5) {
            hashed.push(c);
        } else {
            hashed.push('#');
        }
    }
    return hashed;
}

pub fn clever_maskify(cc: &str) -> String {
    if cc.len() > 4 {
        "#".repeat(cc.len() - 4) + &cc[cc.len() - 4..]
        // repeat # until reaching last 4 characters
        // then add the last four digits
    } else {
        cc.to_string()
    }
}
