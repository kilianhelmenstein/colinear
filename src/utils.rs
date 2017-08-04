pub fn index_in_string_equals_to(string: &str, index: &u32, wanted_char: &char) -> bool {
    match string.chars().nth(*index as usize) {
        Some(actual_char) if actual_char == *wanted_char => true,
        _ => false,
    }
}
