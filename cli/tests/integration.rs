use assert_cmd::prelude::*;
use assert_cmd::Command;
use predicates::prelude::*;

const CLI_BIN: &str = "axiom";

/// Helper to locate the examples folder relative to the CLI crate.
fn example_path(name: &str) -> String {
    format!("../examples/{}", name)
}

#[test]
fn taint_pass_example_runs() -> Result<(), Box<dyn std::error::Error>> {
    Command::cargo_bin(CLI_BIN)?
        .args(&["run", &example_path("taint_pass.axo")])
        .assert()
        .success()
        .stdout(predicate::str::is_empty());
    Ok(())
}

#[test]
fn taint_fail_example_errors() -> Result<(), Box<dyn std::error::Error>> {
    // Feed the script over stdin (so we know exactly what gets parsed)
    let script = "execute_command(read_stdin());";
    Command::cargo_bin(CLI_BIN)?
        .args(&["check", "-"])
        .write_stdin(script)
        .assert()
        .failure()
        .stderr(predicate::str::contains("tainted"));
    Ok(())
}


#[test]
fn nested_calls_work() -> Result<(), Box<dyn std::error::Error>> {
    let script = r#"
        let u = read_stdin();
        let c = sanitize_shell_input(u);
        execute_command(c);
    "#;
    // write a temp file or inline via stdin:
    let mut cmd = Command::cargo_bin(CLI_BIN)?;
    cmd.arg("check")
       .arg("-");        // use stdin
    cmd.write_stdin(script);
    cmd.assert().success().stdout(predicate::str::contains("OK"));
    Ok(())
}
