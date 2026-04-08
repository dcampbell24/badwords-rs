# badwords-rs

An implementation of filtering based on [badwords.csv](badwords.csv).

## Severity

This tells the consumer *how* bad the word actually is, for instance if it's
just something you wouldn't say in polite conversation, or something that
should never be uttered in any circumstances.

The scale I've used here is just three numbers to avoid too much discussion:

* 0: No data available
* 1: Sometimes an acceptable word to say to your parents
* 2: Sometimes an acceptable word to say to friends of same age
* 3: Never acceptable to use
