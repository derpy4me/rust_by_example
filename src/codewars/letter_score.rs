pub fn printer_error(s: &str) -> String {
    format!("{}/{}", s.chars().filter(|&c| c > 'm').count(), s.len())
}
