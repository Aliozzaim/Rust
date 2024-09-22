pub fn get_full_name(first: &str, last: &str) -> String {
    let full_name: String = format!("{1} {0}", first, last);
    return full_name;
}