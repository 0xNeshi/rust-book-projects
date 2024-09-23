const CONSONANTS: &str = "bcdfghjklmnpqrstvwxyzBCDFGHJKLMNPQRSTVWXYZ";
const VOWELS: &str = "aeiouAEIOU";

pub fn to_pig_latin(input: &str) -> String {
    let mut result: Vec<String> = vec![];

    for word in input.split_whitespace() {
        let mut chars = word.chars();
        if let Some(first) = chars.next() {
            let mut word = word;
            let mut punctuation: Option<char> = None;
            if let Some(last) = chars.next_back() {
                if last.is_ascii_punctuation() {
                    word = &word[..word.len() - 1];
                    punctuation = Some(last);
                }
            }
            if VOWELS.contains(first) {
                let mut value = format!("{word}hay");
                if let Some(punctuation) = punctuation {
                    value.push(punctuation);
                }
                result.push(value);
            } else {
                let capitalized = is_capitalized(word);
                let suffix: String = word
                    .chars()
                    .take_while(|ch| CONSONANTS.contains(*ch))
                    .map(|ch| ch.to_lowercase().next().unwrap())
                    .collect();
                let mut prefix = word[suffix.len()..].to_string();
                if capitalized {
                    prefix = capitalize_first(&prefix);
                }
                let mut value = format!("{prefix}{suffix}ay");
                if let Some(punctuation) = punctuation {
                    value.push(punctuation);
                }
                result.push(value);
            }
        }
    }

    result.join(" ")
}

fn is_capitalized(s: &str) -> bool {
    s.chars().next().unwrap().is_uppercase()
}

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    let first_char = chars.next().map(|c| c.to_uppercase().collect::<String>());
    let rest: String = chars.collect();
    match first_char {
        Some(first) => format!("{}{}", first, rest),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_string() {
        assert_eq!(to_pig_latin(""), "")
    }

    #[test]
    fn single_consonant() {
        assert_eq!(to_pig_latin("t"), "tay")
    }

    #[test]
    fn single_vowel() {
        assert_eq!(to_pig_latin("o"), "ohay")
    }

    #[test]
    fn single_words() {
        let test_cases = [
            ("first", "irstfay"),
            ("pig", "igpay"),
            ("latin", "atinlay"),
            ("banana", "ananabay"),
            ("friends", "iendsfray"),
            ("smile", "ilesmay"),
            ("string", "ingstray"),
            ("apple", "applehay"),
            ("eat", "eathay"),
            ("omelet", "omelethay"),
            ("are", "arehay"),
        ];

        for (input, expected) in test_cases {
            assert_eq!(to_pig_latin(input), expected)
        }
    }

    #[test]
    fn sentence() {
        assert_eq!(
            to_pig_latin("What is this weird language? I don't understand it. Aren't you French?"),
            "Atwhay ishay isthay eirdway anguagelay? Ihay on'tday understandhay ithay. Aren'thay ouyay Enchfray?"
        )
    }
}
