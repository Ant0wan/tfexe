use std::env;
use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let output = Command::new("/usr/local/bin/tfswitch")
        .output()
        .expect("failed to get Terraform binary");

    let status = Command::new("terraform")
        .args(args)
        .status()
        .expect("failed to get Terraform binary");

    if output.status.success() && status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command failed",
        ))
    }
}
