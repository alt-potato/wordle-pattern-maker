use std::{collections::HashMap, fs};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum PatternState {
    Green,  // correct letter, correct position
    Yellow, // correct letter, wrong position
    Grey,   // incorrect letter
}

impl From<char> for PatternState {
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'G' => PatternState::Green,
            'Y' => PatternState::Yellow,
            'X' => PatternState::Grey,
            _ => panic!("Invalid pattern character: '{}'", c),
        }
    }
}

// application config
struct Config {
    wordlist_path: String,
    solution: String,
    pattern: String,
}

fn main() {
    let config: Config = Config {
        wordlist_path: String::from("wordlist.txt"),
        solution: String::from("amuse"),
        pattern: String::from(
            r#"
            XXXXX
            XYXGG
            GGGGG
        "#,
        ),
    };

    // load wordlist
    // filter to only include words with the correct length just in case, although it should always be 5
    let wordlist: Vec<String> = match load_wordlist(&config.wordlist_path, config.solution.len()) {
        Ok(words) => words,
        Err(e) => {
            eprintln!("Failed to load wordlist: {}.", e);
            return;
        }
    };

    if wordlist.is_empty() {
        eprintln!("No words found in wordlist.");
        return;
    }

    // parse patterns of every word :)
    let mut pattern_map: HashMap<Vec<PatternState>, Vec<String>> = HashMap::new();
    for word in &wordlist {
        let pattern: Vec<PatternState> = calculate_pattern(&word, &config.solution);
        pattern_map.entry(pattern).or_default().push(word.clone());
    }

    // get solutions for each pattern we want
    let patterns: Vec<Vec<PatternState>> = parse_pattern(&config.pattern);
    let mut possible: bool = true;
    for pattern in patterns {
        if let Some(solutions) = pattern_map.get(&pattern) {
            println!(
                "Possible solutions for pattern {}:",
                pattern
                    .iter()
                    .map(|s| format!("{:?}", s))
                    .collect::<Vec<String>>()
                    .join("")
            );
            if let Some(first_solution) = solutions.first() {
                println!("  {}", first_solution);
                if solutions.len() > 1 {
                    println!("  (and {} others)", solutions.len() - 1);
                }
            }
        } else {
            println!(
                "No possible solutions found for pattern {}.",
                pattern
                    .iter()
                    .map(|s| format!("{:?}", s))
                    .collect::<Vec<String>>()
                    .join("")
            );
            possible = false;
        }
    }

    if !possible {
        println!("Some patterns have no possible solutions. :(");
    }
}

/// Loads a wordlist from a file at the given path, returning a vector of word strings. Results are filtered to only
/// include those that are composed of only ASCII alphabetic characters and have the specified length.
/// If the file does not exist, or there is an error reading the file, an error is returned.
fn load_wordlist(path: &str, word_length: usize) -> Result<Vec<String>, std::io::Error> {
    let content: String = fs::read_to_string(path)?;
    let words: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_lowercase())
        .filter(|s| s.len() == word_length && s.chars().all(|c| c.is_ascii_alphabetic()))
        .collect();
    Ok(words)
}

/// Parses a pattern string into a vector of vectors of PatternState enums.
/// Expected format:
///   XXXXX
///   YYYYY
///   GGGGG
fn parse_pattern(pattern_str: &str) -> Vec<Vec<PatternState>> {
    pattern_str
        .lines()
        .map(|line| line.trim()) // trim whitespace from each line
        .filter(|line| !line.is_empty()) // filter out now-empty lines
        .map(|line| line.chars().map(PatternState::from).collect())
        .collect()
}

/// Calculates the pattern for a given guess and solution.
fn calculate_pattern(guess: &str, solution: &str) -> Vec<PatternState> {
    let word_len = solution.len();
    let mut pattern = vec![PatternState::Grey; word_len];
    let mut solution_chars: Vec<char> = solution.chars().collect();
    let mut guess_chars: Vec<char> = guess.chars().collect();

    // first pass for G (correct letter, correct position)
    for i in 0..word_len {
        if guess_chars[i] == solution_chars[i] {
            pattern[i] = PatternState::Green;
            // mark as used
            solution_chars[i] = '\0';
            guess_chars[i] = '\0';
        }
    }

    // second pass for Y (correct letter, wrong position)
    for i in 0..word_len {
        if guess_chars[i] != '\0' {
            if let Some(pos) = solution_chars.iter().position(|&c| c == guess_chars[i]) {
                pattern[i] = PatternState::Yellow;
                solution_chars[pos] = '\0'; // mark as used
            }
        }
    }

    pattern
}
