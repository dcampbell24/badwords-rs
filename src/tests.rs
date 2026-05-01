// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 David Campbell <dcampbell24@gmail.com>

#![cfg(test)]

use crate::{Censor, INTENSE, MILD, MODERATE};

#[test]
fn test_censoring_words() {
    let censor = Censor::default();

    assert_eq!("**** ******** ***", censor.words("dick dickhead fag", MILD));

    assert_eq!(
        "dick ******** ***",
        censor.words("dick dickhead fag", MODERATE)
    );

    assert_eq!(
        "dick dickhead ***",
        censor.words("dick dickhead fag", INTENSE)
    );
}
