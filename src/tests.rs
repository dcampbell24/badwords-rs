// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: 2026 David Campbell <dcampbell24@gmail.com>

#![cfg(test)]

use crate::{Censor, INTENSE, MILD, MODERATE};

#[test]
fn censor() {
    let censor = Censor::default();

    assert_eq!(
        "**** ******** ***",
        censor.censor("dick dickhead fag", MILD)
    );

    assert_eq!(
        "dick ******** ***",
        censor.censor("dick dickhead fag", MODERATE)
    );

    assert_eq!(
        "dick dickhead ***",
        censor.censor("dick dickhead fag", INTENSE)
    );
}
