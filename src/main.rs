use std::env;
use std::io::Result;
use std::process::{exit, Child, Command, ExitStatus};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let mut tfswitch: Child = Command::new("tfswitch").spawn()?;
    let tfswitch_exit_status: ExitStatus = tfswitch.wait()?;
    if !tfswitch_exit_status.success() {
        exit(tfswitch_exit_status.code().unwrap_or(1));
    }

    let mut terraform: Child = Command::new("terraform").args(args).spawn()?;
    let terraform_exit_status: ExitStatus = terraform.wait()?;
    if !terraform_exit_status.success() {
        exit(terraform_exit_status.code().unwrap_or(1));
    }

    Ok(())
}
