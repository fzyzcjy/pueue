use std::process::{Command, Stdio};

use anyhow::Result;
use assert_cmd::cargo::CommandCargoExt;

use crate::helper::*;

/// Spin up the daemon in daemonized mode.
/// The initial command should exit right away and a forked daemon should be started.
#[tokio::test]
async fn test_fork() -> Result<()> {
    let (settings, _tempdir) = daemon_base_setup()?;
    let shared = &settings.shared;

    let mut child = Command::cargo_bin("pueued")?
        .arg("--config")
        .arg("--daemonize")
        .arg(shared.pueue_directory().join("pueue.yml").to_str().unwrap())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    // The daemon should exit by itself immediately.
    let result = child.try_wait();
    println!("{result:?}");
    assert!(
        matches!(result, Ok(Some(_))),
        "Daemon didn't exit despite --daemonize"
    );
    let code = result.unwrap().unwrap();
    assert!(
        matches!(code.code(), Some(0)),
        "Daemon didn't exit --daemonize successfull"
    );

    // The forked daemon should be up and running and we should be able to get some state.
    let _state = get_state(shared).await?;

    // Kill the daemon gracefully and wait for it to shut down.
    assert_success(shutdown_daemon(&settings.shared).await?);
    // Sleep for 500ms and give the daemon time to shut down
    sleep_ms(500).await;

    // The forked daemon should've now shut down.
    let result = get_state(shared).await;
    assert!(
        result.is_err(),
        "The forked daemon is still there after shutdown."
    );

    Ok(())
}
