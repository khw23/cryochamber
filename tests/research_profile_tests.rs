//! Real CLI + socket + daemon regression, isolated from providers and OS services.
use cryochamber::{
    config::{config_path, save_config, CryoConfig},
    todo::TodoFile,
};
use std::{
    fs,
    process::{Command, Stdio},
    time::{Duration, Instant},
};

fn run_daemon(root: &std::path::Path, registry: &std::path::Path) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_cryo"))
        .arg("daemon")
        .current_dir(root)
        .env("XDG_STATE_HOME", registry)
        .env("CRYO_NO_SERVICE", "1")
        .env("TEST_CRYO_AGENT", env!("CARGO_BIN_EXE_cryo-agent"))
        .stdout(Stdio::null())
        .stderr(Stdio::from(
            fs::File::create(root.join("test-daemon-stderr")).unwrap(),
        ))
        .spawn()
        .unwrap();
    let deadline = Instant::now() + Duration::from_secs(15);
    loop {
        if let Some(status) = child.try_wait().unwrap() {
            assert!(
                status.success(),
                "daemon failed: {status}: {}",
                fs::read_to_string(root.join("test-daemon-stderr")).unwrap()
            );
            break;
        }
        if Instant::now() >= deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("research pause did not stop dispatch");
        }
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[test]
fn real_progress_pause_preserves_todo_and_explicit_restart_answers_human() {
    // Short Unix socket path, also on macOS. No real HOME/config/services touched.
    let temp = tempfile::Builder::new()
        .prefix("cryo-research-")
        .tempdir_in("/tmp")
        .unwrap();
    let root = temp.path();
    let registry = root.join("registry");
    fs::write(
        root.join("plan.md"),
        "# Test\nSynthetic IPC exercise, no scientific calculations.\n",
    )
    .unwrap();
    fs::write(
        root.join("mock.sh"),
        r#"#!/bin/sh
set -eu
case "$1" in *'research_notifications = true'*) ;; *) exit 9 ;; esac
printf '%s\n' 'What the synthetic result means' | "$TEST_CRYO_AGENT" progress --stdin
printf '%s\n' 'Updated synthetic interpretation' | "$TEST_CRYO_AGENT" progress --stdin
"$TEST_CRYO_AGENT" pause --summary 'Synthetic safe boundary'
"#,
    )
    .unwrap();
    let config = CryoConfig {
        agent: "/bin/sh ./mock.sh".into(),
        research_notifications: true,
        max_session_duration: 10,
        watch_dirs: vec![],
        ..CryoConfig::default()
    };
    save_config(&config_path(root), &config).unwrap();
    let prepared = cryochamber::lifecycle::prepare_start(root, Default::default()).unwrap();
    cryochamber::state::save_state(&cryochamber::state::state_path(root), &prepared.state).unwrap();
    let todos = TodoFile::new(root.join("todo.json"));
    todos
        .add("Preserve scientific task".into(), "2000-01-01T00:00".into())
        .unwrap();
    run_daemon(root, &registry);
    let items = todos.items().unwrap();
    assert_eq!(items.len(), 2);
    assert!(items[0].done && !items[0].claimed);
    assert!(!items[1].done && !items[1].claimed);
    assert_eq!(items[1].text, "Preserve scientific task");
    let progress =
        cryochamber::progress::ProgressState::load(&root.join(".cryo/progress-state.json"))
            .unwrap();
    assert_eq!(
        progress.pending.as_deref(),
        Some("Updated synthetic interpretation\n")
    );
    let before = cryochamber::message::read_outbox(root).unwrap();
    // Clock-independent: quiet hours send none; daytime sends only the first.
    assert!(before.len() <= 1);
    assert!(before.iter().all(|(_, m)| m.from == "agent"));
    let state: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join("timer.json")).unwrap()).unwrap();
    assert!(state["pid"].is_null());
    assert_eq!(state["session_number"], 1);
    assert_ne!(state["previous_session_crashed"], true);

    let message = cryochamber::message::Message {
        from: "human".into(),
        subject: "Question".into(),
        body: "Please explain".into(),
        timestamp: chrono::Local::now().naive_local(),
        metadata: Default::default(),
        is_question: false,
    };
    cryochamber::message::write_message(root, "inbox", &message).unwrap();
    fs::write(
        root.join("mock.sh"),
        r#"#!/bin/sh
set -eu
"$TEST_CRYO_AGENT" receive
"$TEST_CRYO_AGENT" send 'Direct answer to the operator'
"$TEST_CRYO_AGENT" pause --summary 'Back to discussion'
"#,
    )
    .unwrap();
    run_daemon(root, &registry); // Explicit restart, no scheduler-created retry.
    let messages = cryochamber::message::read_outbox(root).unwrap();
    assert_eq!(messages.len(), before.len() + 1);
    assert!(messages
        .iter()
        .any(|(_, m)| m.body == "Direct answer to the operator"));
    let items = todos.items().unwrap();
    assert_eq!(items.iter().filter(|t| !t.done).count(), 1);
    assert!(items.iter().all(|t| !t.text.contains("attempt")));
}
