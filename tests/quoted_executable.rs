//! Stage 18: executing an executable whose filename contains spaces or quotes.
//!
//! These are integration tests: they run the built shell as a subprocess with a
//! fixture directory prepended to PATH, feed it commands on stdin, and assert on
//! stdout. The lexer is exercised end to end, which is the point of the stage --
//! the risk it guards against is re-splitting the command name after lexing.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

/// A temp directory that deletes itself when the test ends.
struct Fixture {
    dir: PathBuf,
}

impl Fixture {
    fn new(label: &str) -> Self {
        let dir = std::env::temp_dir().join(format!(
            "rust-shell-test-{}-{}",
            std::process::id(),
            label
        ));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("create fixture dir");
        Fixture { dir }
    }

    /// Write an executable shell script into the fixture directory.
    fn exe(&self, name: &str, body: &str) -> &Self {
        let path = self.dir.join(name);
        fs::write(&path, format!("#!/bin/sh\n{body}\n")).expect("write exe");

        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&path).expect("stat exe").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&path, perms).expect("chmod exe");

        self
    }

    fn path(&self) -> &Path {
        &self.dir
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

/// Run the shell with `bin_dir` on PATH, feeding `input` on stdin.
///
/// `exit` is appended so these tests terminate on an explicit command rather
/// than on EOF. EOF handling has its own test below; keeping it out of this
/// helper means a regression there fails one test instead of hanging all of them.
fn run_shell(bin_dir: &Path, input: &str) -> String {
    let path_var = match std::env::var("PATH") {
        Ok(existing) => format!("{}:{}", bin_dir.display(), existing),
        Err(_) => bin_dir.display().to_string(),
    };

    let mut child = Command::new(env!("CARGO_BIN_EXE_rust-shell"))
        .env("PATH", path_var)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn shell");

    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(format!("{input}\nexit\n").as_bytes())
        .expect("write stdin");

    let out = child.wait_with_output().expect("wait for shell");
    String::from_utf8_lossy(&out.stdout).into_owned()
}

#[test]
fn single_quoted_executable_with_spaces() {
    let f = Fixture::new("single");
    f.exe("exe with space", r#"echo "ran with args: $*""#);

    let out = run_shell(f.path(), "'exe with space' file.md");
    assert!(
        out.contains("ran with args: file.md"),
        "expected the quoted executable to run; got:\n{out}"
    );
}

#[test]
fn double_quoted_executable_with_spaces() {
    let f = Fixture::new("double");
    f.exe("exe with space", r#"echo "ran with args: $*""#);

    let out = run_shell(f.path(), "\"exe with space\" file.md");
    assert!(
        out.contains("ran with args: file.md"),
        "expected the quoted executable to run; got:\n{out}"
    );
}

#[test]
fn backslash_escaped_executable_with_spaces() {
    let f = Fixture::new("backslash");
    f.exe("exe with space", r#"echo "ran with args: $*""#);

    let out = run_shell(f.path(), r"exe\ with\ space file.md");
    assert!(
        out.contains("ran with args: file.md"),
        "expected the escaped executable to run; got:\n{out}"
    );
}

#[test]
fn executable_name_containing_double_quotes() {
    let f = Fixture::new("dqname");
    f.exe(r#"exe with "quotes""#, r#"echo "ran dq-name: $*""#);

    let out = run_shell(f.path(), r#"'exe with "quotes"' file.md"#);
    assert!(
        out.contains("ran dq-name: file.md"),
        "expected the executable with quotes in its name to run; got:\n{out}"
    );
}

#[test]
fn executable_name_containing_single_quotes() {
    let f = Fixture::new("sqname");
    f.exe("exe with 'single quotes'", r#"echo "ran sq-name: $*""#);

    let out = run_shell(f.path(), "\"exe with 'single quotes'\" file.md");
    assert!(
        out.contains("ran sq-name: file.md"),
        "expected the executable with quotes in its name to run; got:\n{out}"
    );
}

#[test]
fn multiple_arguments_survive_quoting() {
    let f = Fixture::new("multiarg");
    f.exe("exe with space", r#"echo "count=$# args=$*""#);

    let out = run_shell(f.path(), "'exe with space' one 'two three' four");
    assert!(
        out.contains("count=3 args=one two three four"),
        "expected three arguments with the quoted one intact; got:\n{out}"
    );
}

/// Closing stdin must terminate the shell. It previously busy-looped forever:
/// `read_line` returned `Ok(0)`, the empty line lexed to no words, and the loop
/// went straight back to reading. Guarded by a timeout so a regression fails
/// rather than hanging the suite.
#[test]
fn closing_stdin_exits_the_shell() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_rust-shell"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn shell");

    // Write one command and close stdin -- no `exit`.
    child
        .stdin
        .take()
        .expect("stdin")
        .write_all(b"echo hi\n")
        .expect("write stdin");

    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        match child.try_wait().expect("try_wait") {
            Some(status) => {
                assert!(status.success(), "shell exited with {status} on EOF");
                return;
            }
            None if Instant::now() >= deadline => {
                let _ = child.kill();
                panic!("shell did not exit within 5s of stdin closing");
            }
            None => std::thread::sleep(Duration::from_millis(20)),
        }
    }
}
