use crate::fixtures::surmig;

mod fixtures;

#[test]
fn help_cmd() {
    let cmd = surmig().args(["help"]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!("help_cmd/help.stdout"))
        .stderr_eq("");
}

#[test]
fn help_opt() {
    let cmd = surmig().args(["--help"]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!("help_cmd/help.stdout"))
        .stderr_eq("");
}

#[test]
fn help_short_opt() {
    let cmd = surmig().args(["-h"]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!("help_cmd/help.stdout"))
        .stderr_eq("");
}

#[test]
fn help_no_args() {
    let cmd = surmig();

    cmd.assert()
        .code(2)
        .stdout_eq("")
        .stderr_eq(snapbox::file!("help_cmd/help.stdout"));
}
