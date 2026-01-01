// SPDX-License-Identifier: Apache-2.0

//! Integration tests for CLI commands

use std::process::Command;

/// Helper to run CLI command via cargo
fn run_cli(args: &[&str]) -> std::process::Output {
    Command::new("cargo")
        .arg("run")
        .arg("-p")
        .arg("audio-ninja-cli")
        .arg("--")
        .args(args)
        .output()
        .expect("Failed to execute CLI")
}

#[test]
fn test_cli_help() {
    let output = run_cli(&["--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Audio Ninja command-line interface"));
    assert!(stdout.contains("status"));
    assert!(stdout.contains("info"));
    assert!(stdout.contains("speaker"));
    assert!(stdout.contains("layout"));
    assert!(stdout.contains("transport"));
    assert!(stdout.contains("calibration"));
    assert!(stdout.contains("stats"));
}

#[test]
fn test_cli_version() {
    let output = run_cli(&["--version"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("audio-ninja"));
}

#[test]
fn test_speaker_help() {
    let output = run_cli(&["speaker", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Speaker management"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("discover"));
    assert!(stdout.contains("get"));
    assert!(stdout.contains("remove"));
    assert!(stdout.contains("stats"));
}

#[test]
fn test_layout_help() {
    let output = run_cli(&["layout", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Layout configuration"));
    assert!(stdout.contains("get"));
    assert!(stdout.contains("set"));
}

#[test]
fn test_transport_help() {
    let output = run_cli(&["transport", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Transport control"));
    assert!(stdout.contains("play"));
    assert!(stdout.contains("pause"));
    assert!(stdout.contains("stop"));
    assert!(stdout.contains("status"));
}

#[test]
fn test_calibration_help() {
    let output = run_cli(&["calibration", "--help"]);

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Calibration"));
    assert!(stdout.contains("start"));
    assert!(stdout.contains("status"));
    assert!(stdout.contains("apply"));
}

#[test]
fn test_daemon_url_flag() {
    // Test with non-existent daemon - should fail to connect but parse args correctly
    let output = run_cli(&["--daemon", "http://localhost:9999", "status"]);

    // Should fail because daemon isn't running
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Error should be about connection, not argument parsing
    assert!(stderr.contains("Failed to send request") || stderr.contains("connect"));
}

#[test]
fn test_invalid_command() {
    let output = run_cli(&["invalid-command"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unrecognized subcommand") || stderr.contains("error"));
}

#[test]
fn test_speaker_get_requires_uuid() {
    let output = run_cli(&["speaker", "get"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required") || stderr.contains("argument"));
}

#[test]
fn test_layout_set_requires_preset() {
    let output = run_cli(&["layout", "set"]);

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required") || stderr.contains("argument"));
}
