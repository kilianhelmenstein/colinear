use super::super::tokens::Token;

pub fn n_following_values<'a>(stream: &'a [Token], min: &usize, max: &usize) -> Result<(&'a [Token], Vec<String>), &'static str> {
    append_n_following_values(stream, Vec::new(), min, max)
}

pub fn append_n_following_values<'a>(
    stream: &'a [Token],
    mut appended: Vec<String>,
    min: &usize, max: &usize) -> Result<(&'a [Token], Vec<String>), &'static str> {

    let got_max_number_of_values = *max == 0;
    if got_max_number_of_values {
        return Ok((stream, appended));
    }

    if stream.len() > 0 {
        if let Token::Value(ref token_content) = stream[0] {
            appended.push(token_content.clone());
            append_n_following_values(stream, appended, &(*min-1), &(*max-1))
        } else {
            Ok((stream, appended))
        }
    } else {
        let got_min_number_of_values = *min == 0;
        if got_min_number_of_values {
            Ok((stream, appended))
        } else {
            Err("No value left")
        }
    }
}
