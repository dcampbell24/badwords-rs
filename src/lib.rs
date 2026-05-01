// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 David Campbell <dcampbell24@gmail.com>

pub mod tests;

use std::{collections::HashMap, io::Cursor};

use serde::Deserialize;

const BAD_WORDS: &str = include_str!("../badwords.csv");
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

pub struct Censor {
    words: HashMap<String, u8>,
}

impl Default for Censor {
    fn default() -> Self {
        let mut words = HashMap::new();

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(Cursor::new(BAD_WORDS));

        for result in reader.deserialize() {
            let record: BadWords = result.unwrap();
            for word in record.words.split('/') {
                words.insert(word.to_string(), record.severity);
            }
        }

        Self { words }
    }
}

impl Censor {
    fn censor_word(&self, word: &str, severity: u8) -> bool {
        if let Some(s) = self.words.get(word)
            && *s >= severity
        {
            true
        } else {
            false
        }
    }

    #[must_use]
    pub fn censor(&self, words: &str, severity: u8) -> String {
        let mut censored = Vec::new();

        for word in words.split_whitespace() {
            if self.censor_word(word, severity) {
                censored.push("*".repeat(word.len()));
            } else {
                censored.push(word.to_string());
            }
        }

        censored.join(" ")
    }
}
