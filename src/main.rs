use std::env;
use std::io::Result;
use std::process::Command;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();

    let _ = Command::new("/usr/local/bin/tfswitch")
        .status()
        .expect("failed to get terraform binary of the specified version");

    let status = Command::new("terraform")
        .args(args)
        .status()
        .expect("failed to execute terraform");

    if status.success() && status.success() {
        Ok(())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Command failed",
        ))
    }
}
