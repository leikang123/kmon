use std::io::{self, Write};
use std::process::Command;

/**
 * Execute a operating system command and return its output.
 *
 * @param  cmd
 * @param  cmd_args
 * @return Result
 */
pub fn exec_cmd(cmd: &str, cmd_args: &[&str]) -> Result<String, String> {
    let output = Command::new(cmd)
        .args(cmd_args)
        .output()
        .expect("failed to execute command");
    /* Write error output to stderr stream. */
    io::stderr().write_all(&output.stderr).unwrap();
    if output.status.success() {
        Ok((String::from_utf8(output.stdout).expect("not UTF-8"))
            .trim_end()
            .to_string())
    } else {
        Err(format!("{} {}", cmd, cmd_args.join(" ")))
    }
}

/**
 * Unit tests.
 */
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_exec_cmd() {
        assert_eq!("test", exec_cmd("printf", &["test"]).unwrap());
        assert_eq!(
            "true",
            exec_cmd("sh", &["-c", "test 10 -eq 10 && echo 'true'"]).unwrap()
        );
    }
}
