use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use assert_cmd::prelude::*;
use rstest::rstest;

// Make sure the completion file generation succeeds.
#[rstest]
#[case("zsh")]
#[case("elvish")]
#[case("bash")]
#[case("fish")]
#[case("power-shell")]
#[case("nushell")]
#[test]
fn autocompletion_generation(#[case] shell: &'static str) -> Result<()> {
    let output = Command::cargo_bin("pueue")?
        .arg("completions")
        .arg(shell)
        .arg("./")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(env!("CARGO_TARGET_TMPDIR"))
        .output()
        .context(format!("Failed to run completion generation for {shell}:"))?;

    assert!(
        output.status.success(),
        "Completion for {shell} didn't finish successfully."
    );

    Ok(())
}

// Make sure the completion file stream to stdout succeeds.
#[rstest]
#[case("zsh")]
#[case("elvish")]
#[case("bash")]
#[case("fish")]
#[case("power-shell")]
#[case("nushell")]
#[test]
fn autocompletion_generation_stdout(#[case] shell: &'static str) -> Result<()> {
    let output = Command::cargo_bin("pueue")?
        .arg("completions")
        .arg(shell)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(env!("CARGO_TARGET_TMPDIR"))
        .output()
        .context(format!("Failed to run completion generation for {shell}:"))?;

    assert!(
        output.status.success(),
        "Completion for {shell} didn't finish successfully."
    );

    Ok(())
}
