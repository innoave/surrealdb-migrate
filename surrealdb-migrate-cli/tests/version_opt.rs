mod fixtures;

use fixtures::surmig;

#[test]
fn version_opt() {
    let version = env!("CARGO_PKG_VERSION");

    let cmd = surmig().arg("--version");

    cmd.assert()
        .code(0)
        .stdout_eq(format!("surmig {version}\n"))
        .stderr_eq("");
}

#[test]
fn version_short_opt() {
    let version = env!("CARGO_PKG_VERSION");

    let cmd = surmig().arg("-V");

    cmd.assert()
        .code(0)
        .stdout_eq(format!("surmig {version}\n"))
        .stderr_eq("");
}
