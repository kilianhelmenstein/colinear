pub fn index_in_string_equals_to(index: &u32, string: &str, wanted_char: &char) -> bool {
    match string.chars().nth(*index as usize) {
        Some(actual_char) if actual_char == *wanted_char => true,
        _ => false,
    }
}
