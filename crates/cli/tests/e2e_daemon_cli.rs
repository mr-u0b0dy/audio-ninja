// SPDX-License-Identifier: Apache-2.0

use std::io::Read;
use std::net::{TcpListener, TcpStream};
use std::process::{Child, Command, Stdio};
use std::thread::sleep;
use std::time::{Duration, Instant};

fn pick_free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

fn wait_for_port(host: &str, port: u16, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if TcpStream::connect((host, port)).is_ok() {
            return;
        }
        sleep(Duration::from_millis(100));
    }
    panic!("daemon did not start on {}:{} within {:?}", host, port, timeout);
}

fn spawn_daemon(port: u16) -> Child {
    let mut child = Command::new("cargo")
        .args([
            "run",
            "-p",
            "audio-ninja-daemon",
            "--",
            "--bind",
            "127.0.0.1",
            "--port",
            &port.to_string(),
        ])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn daemon");

    // Wait until the TCP port is accepting connections
    wait_for_port("127.0.0.1", port, Duration::from_secs(10));
    child
}

fn run_cli(args: &[&str]) -> String {
    let output = Command::new("cargo")
        .args(["run", "-p", "audio-ninja-cli", "--"])
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .expect("run cli");

    if !output.status.success() {
        let mut stderr = String::new();
        stderr.push_str(&String::from_utf8_lossy(&output.stderr));
        panic!("CLI failed: {}", stderr);
    }

    String::from_utf8_lossy(&output.stdout).to_string()
}

#[test]
fn e2e_status_and_info() {
    let port = pick_free_port();
    let mut daemon = spawn_daemon(port);
    let base = format!("http://127.0.0.1:{}", port);

    let status = run_cli(&["--daemon", &base, "status"]);
    assert!(status.contains("\"status\""));

    let info = run_cli(&["--daemon", &base, "info"]);
    assert!(info.contains("features"));

    let _ = daemon.kill();
}

#[test]
fn e2e_transport_flow() {
    let port = pick_free_port();
    let mut daemon = spawn_daemon(port);
    let base = format!("http://127.0.0.1:{}", port);

    run_cli(&["--daemon", &base, "transport", "play"]);
    let s1 = run_cli(&["--daemon", &base, "transport", "status"]);
    assert!(s1.contains("\"Playing\""));

    run_cli(&["--daemon", &base, "transport", "pause"]);
    let s2 = run_cli(&["--daemon", &base, "transport", "status"]);
    assert!(s2.contains("\"Paused\""));

    run_cli(&["--daemon", &base, "transport", "stop"]);
    let s3 = run_cli(&["--daemon", &base, "transport", "status"]);
    assert!(s3.contains("\"Stopped\""));

    let _ = daemon.kill();
}

#[test]
fn e2e_layout_set_get() {
    let port = pick_free_port();
    let mut daemon = spawn_daemon(port);
    let base = format!("http://127.0.0.1:{}", port);

    run_cli(&["--daemon", &base, "layout", "set", "stereo"]);
    let layout = run_cli(&["--daemon", &base, "layout", "get"]);
    assert!(layout.contains("stereo") || layout.contains("speakers"));

    let _ = daemon.kill();
}

#[test]
fn e2e_speakers_discover_and_list() {
    let port = pick_free_port();
    let mut daemon = spawn_daemon(port);
    let base = format!("http://127.0.0.1:{}", port);

    // Discovery may be a no-op in tests; ensure it returns successfully
    run_cli(&["--daemon", &base, "speaker", "discover"]);
    let list = run_cli(&["--daemon", &base, "speaker", "list"]);
    // Output is JSON array; may be empty
    assert!(list.trim_start().starts_with("[") || list.contains("speakers"));

    let _ = daemon.kill();
}

#[test]
fn e2e_stats() {
    let port = pick_free_port();
    let mut daemon = spawn_daemon(port);
    let base = format!("http://127.0.0.1:{}", port);

    let stats = run_cli(&["--daemon", &base, "stats"]);
    assert!(stats.contains("uptime") || stats.contains("transport_state"));

    let _ = daemon.kill();
}
