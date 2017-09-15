use super::super::tokens::Token;

pub fn n_following_values(mut stream: Box<Iterator<Item=Token>>, min: &usize, max: &usize) -> Result<(Box<Iterator<Item=Token>>, Vec<String>), &'static str> {
    append_n_following_values(stream, Vec::new(), min, max)
}

pub fn append_n_following_values(
    mut stream: Box<Iterator<Item=Token>>,
    mut appended: Vec<String>,
    min: &usize, max: &usize) -> Result<(Box<Iterator<Item=Token>>, Vec<String>), &'static str> {

    let got_max_number_of_values = *max == 0;
    if got_max_number_of_values {
        return Ok((stream, appended));
    }

    if let Some(Token::Value(next_value)) = stream.next() {
        appended.push(next_value);
        append_n_following_values(stream, appended, &(*min-1), &(*max-1))
    } else {
        let got_min_number_of_values = *min == 0;
        if got_min_number_of_values {
            Ok((stream, appended))
        } else {
            Err("No value left")
        }
    }
}
