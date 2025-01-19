mod fixtures;

use fixtures::surmig;

#[test]
fn version_opt() {
    let version = env!("CARGO_PKG_VERSION");

    let mut cmd = surmig();
    cmd.arg("--version");

    cmd.assert()
        .code(0)
        .stdout(format!("surmig {version}\n"))
        .stderr("");
}

#[test]
fn version_short_opt() {
    let version = env!("CARGO_PKG_VERSION");

    let mut cmd = surmig();
    cmd.arg("-V");

    cmd.assert()
        .code(0)
        .stdout(format!("surmig {version}\n"))
        .stderr("");
}
