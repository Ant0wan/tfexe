use std::env;
use std::io::Result;
use std::process::exit;
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let tfswitch = Command::new("tfswitch")
        .status()
        .expect("failed to execute tfswitch");

    let tfswitch_exit_value = tfswitch.code().unwrap_or(1);
    if tfswitch_exit_value != 0 {
        exit(tfswitch_exit_value);
    }

    let terraform = Command::new("terraform")
        .args(args)
        .status()
        .expect("failed to execute terraform");

    let terraform_exit_value = terraform.code().unwrap_or(1);

    if terraform_exit_value != 0 {
        exit(terraform_exit_value);
    }

    Ok(())
}
