use std::{collections::HashMap, fmt, fs};

// valid pattern states
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

impl fmt::Display for PatternState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PatternState::Green => write!(f, "G"),
            PatternState::Yellow => write!(f, "Y"),
            PatternState::Grey => write!(f, "X"),
        }
    }
}

// valid pattern states, with extra filters for queries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum QueryPatternState {
    Base(PatternState),
    AnyValid,
    Any,
}

impl From<char> for QueryPatternState {
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            '?' => QueryPatternState::AnyValid,
            '*' => QueryPatternState::Any,
            other => QueryPatternState::Base(PatternState::from(other)),
        }
    }
}

impl fmt::Display for QueryPatternState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            QueryPatternState::Base(state) => write!(f, "{}", state),
            QueryPatternState::AnyValid => write!(f, "?"),
            QueryPatternState::Any => write!(f, "*"),
        }
    }
}

impl QueryPatternState {
    fn possible_states(&self) -> Vec<PatternState> {
        match self {
            QueryPatternState::Base(state) => vec![*state],
            QueryPatternState::AnyValid => vec![PatternState::Green, PatternState::Yellow],
            QueryPatternState::Any => vec![PatternState::Green, PatternState::Yellow, PatternState::Grey],
        }
    }
}

// application config
struct Config {
    wordlist_path: String,
    solution: String,
    pattern: String,
}

// two approaches:
// 1. get pattern of every word and generate all patterns that match query pattern,
//    then get all words that match those patterns
//    + should use less memory, and only processes each word once (O(w))
//    - query pattern expansion could be expensive (O(q * 3^5), or O(q * 3^l) if pattern length is variable in the future)
// 2. for every word, check if it matches a query pattern,
//    then collect all words that match each query pattern
//    - words must be processed multiple times (O(wq))
//    + simpler access after all words have been processed (O(1))
// 
// winner: 1 (yippee)
fn main() {
    let config: Config = Config {
        wordlist_path: String::from("wordlist.txt"),
        solution: String::from("ideal"),
        pattern: String::from(
            r#"
            ??*??
            ?XXX?
            ???X?
            ?X?X?
            ???X?
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

    // get solutions for each query pattern we want
    let query_patterns: Vec<Vec<QueryPatternState>> = parse_query_pattern(&config.pattern);
    let mut possible: bool = true;
    for query in query_patterns {
        let patterns: Vec<Vec<PatternState>> = expand_query_pattern(&query);
        let mut solutions: Vec<&String> = Vec::new();

        // for each (generated) pattern, get all words that match
        for pattern in patterns {
            if let Some(matches) = pattern_map.get(&pattern) {
                // words for each pattern are guaranteed to be unique
                solutions.extend(matches);
            }
        }

        if solutions.len() > 0 {
            println!(
                "Possible solutions for pattern {}:",
                query_pattern_to_string(&query)
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
                query_pattern_to_string(&query)
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

/// Parses a query pattern string into a vector of vectors of QueryPatternState enums.
/// Expected format:
///   XXXXX
///   YYYYY
///   GGGGG
///   ?????
///   *****
fn parse_query_pattern(pattern_str: &str) -> Vec<Vec<QueryPatternState>> {
    pattern_str
        .lines()
        .map(|line| line.trim()) // trim whitespace from each line
        .filter(|line| !line.is_empty()) // filter out now-empty lines
        .map(|line| line.chars().map(QueryPatternState::from).collect())
        .collect()
}

/// Expands a query pattern into a vector of vectors of PatternState enums, representing all possible patterns
/// that match the query pattern.
fn expand_query_pattern(pattern: &Vec<QueryPatternState>) -> Vec<Vec<PatternState>> {
    let mut results: Vec<Vec<PatternState>> = vec![Vec::new()];

    // for each query state (G, Y, X, ?, *) in query pattern...
    for query_state in pattern {
        let possible_states: Vec<PatternState> = query_state.possible_states();
        
        // ...extend results with each possible state
        let mut new_results: Vec<Vec<PatternState>> = Vec::with_capacity(results.len() * possible_states.len());
        for result in &results {
            for possible_state in &possible_states {
                let mut new_result = result.clone();
                new_result.push(*possible_state);
                new_results.push(new_result);
            }
        }
        results = new_results;
    }

    results
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

/// Converts a vector of QueryPatternState enums into a string representation, with one character per state.
fn query_pattern_to_string(pattern: &Vec<QueryPatternState>) -> String {
    pattern.iter().map(|s| s.to_string()).collect()
}
