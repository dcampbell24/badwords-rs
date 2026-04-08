use badwords_rs::{INTENSE, MILD, MODERATE, censor};

fn main() -> anyhow::Result<()> {
    println!("{}", censor("dick dickhead fag", MILD)?);
    println!("{}", censor("dick dickhead fag", MODERATE)?);
    println!("{}", censor("dick dickhead fag", INTENSE)?);

    Ok(())
}
