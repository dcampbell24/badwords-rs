// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 David Campbell <dcampbell24@gmail.com>

use std::{collections::HashMap, io::Cursor};

use serde::Deserialize;

const BAD_WORDS: &str = include_str!("../bad_words.csv");
pub const MILD: u8 = 1;
pub const MODERATE: u8 = 2;
pub const INTENSE: u8 = 3;

#[derive(Debug, Deserialize, Eq, Hash, PartialEq)]
struct BadWords {
    locale: String,
    words: String,
    translation: String,
    severity: u8,
}

/// # Panics
///
/// If it fails to deserialize `bad_words.csv`.
#[must_use]
pub fn censor(words: &str, severity: u8) -> String {
    let mut bad_words = HashMap::new();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(Cursor::new(BAD_WORDS));

    for result in reader.deserialize() {
        let record: BadWords = result.unwrap();
        for word in record.words.split('/') {
            bad_words.insert(word.to_string(), record.severity);
        }
    }

    let mut censored = Vec::new();
    for word in words.split_whitespace() {
        if let Some(s) = bad_words.get(word)
            && *s >= severity {
                censored.push("*".repeat(word.len()));
            } else {
                censored.push(word.to_string());
            }
    }

    censored.join(" ")
}
