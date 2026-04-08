// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 David Campbell <dcampbell24@gmail.com>

use badwords_rs::{INTENSE, MILD, MODERATE, censor};

fn main() {
    println!("{}", censor("dick dickhead fag", MILD));
    println!("{}", censor("dick dickhead fag", MODERATE));
    println!("{}", censor("dick dickhead fag", INTENSE));
}
