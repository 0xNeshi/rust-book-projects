use regex::Regex;

fn is_consonant(c: char) -> bool {
    let consonant_regex = Regex::new(r"(?i)[bcdfghjklmnpqrstvwxyz]").unwrap();
    consonant_regex.is_match(&c.to_string())
}

// Function to capitalize the first letter of a word and lowercase the rest
fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    if let Some(first_char) = chars.next() {
        first_char.to_uppercase().collect::<String>() + chars.as_str().to_lowercase().as_str()
    } else {
        String::new()
    }
}

// Function to check if the word was capitalized
fn was_capitalized(word: &str) -> bool {
    word.chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
}

// Function to convert a single word to Pig Latin, handling consonant clusters, capitalization, and punctuation
fn word_to_pig_latin(word: &str) -> String {
    let re = Regex::new(r"([a-zA-Z']+)([.,!?;]*)").unwrap(); // Regex to split word and handle apostrophes
    if let Some(captures) = re.captures(word) {
        let prefix = captures.get(1).unwrap().as_str(); // Full word
        let punctuation = captures.get(2).unwrap().as_str(); // Any punctuation

        // Preserve capitalization information
        let capitalized = was_capitalized(prefix);

        // Collect leading consonants
        let mut consonant_prefix = String::new();
        for c in prefix.chars().take_while(|ch| is_consonant(*ch)) {
            consonant_prefix.push(c);
        }

        // Build the remainder of the word after the consonant prefix
        let remainder: String = prefix.chars().skip(consonant_prefix.len()).collect();

        let pig_latin_word = if consonant_prefix.is_empty() {
            // If the word starts with a vowel, just add "hay"
            format!("{}hay", prefix)
        } else {
            // Otherwise, move the consonant prefix to the end and add "ay"
            format!("{}{}ay", remainder, consonant_prefix)
        };

        // Reapply capitalization if the original word was capitalized
        let final_word = if capitalized {
            capitalize_first(&pig_latin_word)
        } else {
            pig_latin_word
        };

        // Return the Pig Latin word with punctuation added back
        format!("{}{}", final_word, punctuation)
    } else {
        word.to_string()
    }
}

// Public function to convert a sentence to Pig Latin
pub fn to_pig_latin(sentence: &str) -> String {
    sentence
        .split_whitespace()
        .map(|word| word_to_pig_latin(word))
        .collect::<Vec<String>>()
        .join(" ")
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
