// A Git hook in Rust that performs spell checking on commit messages.
// If run with -m (or given a commit message file), it offers interactive inline suggestions for misspellings.

use hunspell::Hunspell;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::{self, BufRead, BufReader, Write};
use tempfile::NamedTempFile;
use std::process::exit;

/// Load words to ignore from a `.spellignore` file in the repo root.
/// These words will not be flagged as misspellings.
/// Returns a HashSet of lowercase ignored words for fast lookup.
fn load_ignore_words() -> HashSet<String> {
    let mut ignored = HashSet::new();

    // Try to open `.spellignore` file; ignore errors silently (file may not exist)
    if let Ok(file) = fs::File::open(".spellignore") {
        let reader = BufReader::new(file);
        for line in reader.lines().flatten() {
            // Trim whitespace and convert to lowercase for uniform matching
            let word = line.trim().to_lowercase();
            if !word.is_empty() {
                ignored.insert(word);
            }
        }
    }
    ignored
}

/// Prompt the user to replace a misspelled word interactively.
/// Displays suggestions from Hunspell and reads user input from stdin.
/// Returns Some(replacement) if user enters a replacement word, or None to skip replacement.
fn prompt_replace(word: &str, suggestions: &[String]) -> Option<String> {
    println!("[REPLACE] '{}' - suggestions: {}", word, suggestions.join(", "));
    print!("Enter replacement (or press ENTER to skip): ");
    io::stdout().flush().unwrap(); // Flush to ensure prompt shows immediately

    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let replacement = input.trim();
        if !replacement.is_empty() {
            // Return replacement string if user entered one
            return Some(replacement.to_string());
        }
    }
    None
}

/// Applies inline fixes by scanning the commit message content word by word,
/// checking for misspellings, and prompting the user interactively for replacements.
/// Returns the updated commit message string after applying all accepted fixes.
fn apply_inline_fixes(content: &str, hunspell: &Hunspell, ignore_words: &HashSet<String>) -> String {
    let mut fixed = String::new();

    // Split content by whitespace, process each word
    for word in content.split_whitespace() {
        // Clean word by trimming non-alphabetic characters (punctuation, symbols)
        let clean = word.trim_matches(|c: char| !c.is_alphabetic());

        // Check if word is not empty, not ignored, and is misspelled according to Hunspell
        if !clean.is_empty()
            && !ignore_words.contains(&clean.to_lowercase())
            && !hunspell.check(clean)
        {
            // Get suggestions from Hunspell for the misspelled word
            let suggestions = hunspell.suggest(clean);

            // Prompt user for replacement interactively
            if let Some(replacement) = prompt_replace(clean, &suggestions) {
                // Replace only the cleaned substring (to preserve punctuation) with replacement
                fixed += &word.replace(clean, &replacement);
            } else {
                // User skipped replacement, keep original word
                fixed += word;
            }
        } else {
            // Word is correct or ignored, keep as is
            fixed += word;
        }
        fixed.push(' '); // Re-add the whitespace separator between words
    }

    fixed.trim_end().to_string() // Trim trailing whitespace and return
}

/// Scans the given text and returns a vector of misspelled words.
/// Misspellings exclude any word in the ignore_words set.
/// Words are cleaned by trimming non-alphabetic characters.
fn find_misspellings(text: &str, hunspell: &Hunspell, ignore_words: &HashSet<String>) -> Vec<String> {
    text.split_whitespace()
        // Clean words by trimming punctuation and symbols
        .map(|word| word.trim_matches(|c: char| !c.is_alphabetic()))
        // Filter to only misspelled and non-ignored words
        .filter(|clean| {
            !clean.is_empty()
                && !ignore_words.contains(&clean.to_lowercase())
                && !hunspell.check(clean)
        })
        .map(String::from)
        .collect()
}

/// Creates a Hunspell instance using embedded dictionary files baked into the binary.
/// It writes those bytes to temporary files so Hunspell can read them as normal files.
/// - HUNSPELL_AFF: path to affix file (.aff)
/// - HUNSPELL_DIC: path to dictionary file (.dic)
fn get_hunspell() -> Hunspell {
    // Embed the .aff and .dic files as byte slices
    let aff_data = include_bytes!("assets/en_US.aff");
    let dic_data = include_bytes!("assets/en_US.dic");

    // Write affix file to a temporary file
    let mut aff_file = NamedTempFile::new().expect("Failed to create temp .aff");
    aff_file.write_all(aff_data).expect("Failed to write .aff");
    let aff_path = aff_file.path().to_owned();

    // Write dictionary file to another temporary file
    let mut dic_file = NamedTempFile::new().expect("Failed to create temp .dic");
    dic_file.write_all(dic_data).expect("Failed to write .dic");
    let dic_path = dic_file.path().to_owned();

    // Note: these files are deleted automatically when the NamedTempFile goes out of scope.
    // To keep them alive for the life of the program, we need to leak them (safe here since it's short-lived)
    std::mem::forget(aff_file);
    std::mem::forget(dic_file);

    Hunspell::new(aff_path.to_str().unwrap(), dic_path.to_str().unwrap())
}


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("[ERROR] No commit message file provided.");
        exit(1);
    }

    let commit_msg_path = &args[1];

    // Read the commit message file and filter out comment lines starting with '#'
    let content = match fs::read_to_string(commit_msg_path) {
        Ok(c) => c
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n"),
        Err(e) => {
            eprintln!("[ERROR] Failed to read commit message: {}", e);
            exit(1);
        }
    };


    let ignore_words = load_ignore_words();
    let hunspell = get_hunspell();

    // Initial spellcheck
    let mistakes = find_misspellings(&content, &hunspell, &ignore_words);

    if !mistakes.is_empty() {
        eprintln!("[SPELLCHECK] Found possible spelling mistakes:");
        for word in &mistakes {
            eprintln!("  - {}", word);
        }

        // Prompt user for inline replacements
        let updated = apply_inline_fixes(&content, &hunspell, &ignore_words);
        let _ = fs::write(commit_msg_path, updated);

        let new_content = match fs::read_to_string(commit_msg_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("[ERROR] Failed to re-read commit message: {}", e);
                exit(1);
            }
        };

        // Final check for remaining issues
        let remaining = find_misspellings(&new_content, &hunspell, &ignore_words);
        if !remaining.is_empty() {
            eprintln!("\n[WARNING] Spelling mistakes still found after editing:");
            for word in &remaining {
                eprintln!("  - {}", word);
            }

            // Ask user whether to proceed anyway
            print!("Do you want to proceed with the commit? [y/N]: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            let proceed = {
                io::stdin().read_line(&mut input).is_ok()
                    && !matches!(input.trim().to_lowercase().as_str(), "n" | "no")
            };

            if !proceed {
                eprintln!("[CANCELLED] Commit aborted due to unresolved spelling issues.");
                exit(1);
            }

        }
    }

    // Exit successfully if no issues or user confirms
    exit(0);
}

