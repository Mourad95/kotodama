//! The panic hook installed by `kd-ffi` must not print the panic payload (it can hold message
//! text, docs/legal.md L1/L18). The test re-runs itself as a child process and inspects stderr.

use std::panic::{catch_unwind, AssertUnwindSafe};
use std::process::Command;

const SECRET: &str = "TOP-SECRET-MESSAGE-TEXT";
const CHILD: &str = "KD_PANIC_HOOK_CHILD";

#[test]
fn panic_hook_does_not_leak_the_payload() {
    if std::env::var_os(CHILD).is_some() {
        kd_ffi::install_panic_hook();
        let _ = catch_unwind(AssertUnwindSafe(|| panic!("failed on {SECRET}")));
        return;
    }
    let out = Command::new(std::env::current_exe().unwrap())
        .args([
            "panic_hook_does_not_leak_the_payload",
            "--exact",
            "--nocapture",
            "--test-threads=1",
        ])
        .env(CHILD, "1")
        .output()
        .unwrap();
    let stderr = String::from_utf8_lossy(&out.stderr);
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(out.status.success(), "child failed: {stderr}");
    assert!(
        stderr.contains("details withheld"),
        "hook not active: {stderr}"
    );
    assert!(!stderr.contains(SECRET) && !stdout.contains(SECRET));
}
