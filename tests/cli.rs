use assert_cmd::Command;
use common::input;
use parameterized::{ide, parameterized};

mod common;

mod from_path {
    use super::*;
    ide!();

    #[parameterized(args = {
        &[input(), "--close-after", "10"],
        &["--from-path", input(), "--close-after", "10"],
    })]
    fn from_path(args: &[&str]) {
        let _ = Command::cargo_bin("miniview")
            .expect("MiniView binary not found")
            .args(args)
            .assert()
            .success();
    }
}

mod from_stdin {
    use super::*;

    #[test]
    fn stdin_path() {
        let _ = Command::cargo_bin("miniview")
            .expect("MiniView binary not found")
            .args(&["--from-stdin-path", "--close-after", "1"])
            .write_stdin(input())
            .assert()
            .success();
    }

    #[test]
    fn stdin_bytes() {
        let _ = Command::cargo_bin("miniview")
            .expect("MiniView binary not found")
            .args(&["--from-stdin-bytes", "--close-after", "10"])
            .pipe_stdin(input())
            .expect("Test input file not found")
            .assert()
            .success();
    }
}
