pub fn make_family_friendly(text: &str) -> String {
    // List of replacement pairs (inappropriate -> family friendly)
    let replacements = [
        ("curse_word", "oh dear"),
        ("bad_word", "goodness"),
        // Add more replacements as needed
    ];
    
    let mut result = text.to_string();
    for (inappropriate, friendly) in replacements.iter() {
        result = result.replace(inappropriate, friendly);
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_family_friendly() {
        let input = "Oh curse_word, this is bad_word!";
        let expected = "Oh oh dear, this is goodness!";
        assert_eq!(make_family_friendly(input), expected);
    }
} 