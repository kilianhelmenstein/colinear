
pub fn indexInStringEqualsTo(index: &u32, string: &str, wantedChar: &char) -> bool {
    match string.chars().nth(*index as usize) {
        Some(actualChar) if actualChar == *wantedChar => true,
        _ => false,
    }
}

