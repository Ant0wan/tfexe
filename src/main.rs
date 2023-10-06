use std::env;
use std::process::{Command, ExitStatus};

fn exec(val: &str, args: Vec<String>) -> Result<ExitStatus, std::io::Error> {
    let mut cmd = Command::new(val);
    for arg in args {
        cmd.arg(arg);
    }
    cmd.spawn()?.wait()
}

fn main() -> Result<(), std::io::Error> {
    let args: Vec<String> = env::args().skip(1).collect();
    let tfexe: String = env::var("TFEXE").unwrap_or_else(|_| "tfswitch".to_string());

    match tfexe.as_str() {
        "tfswitch" => {
            let _ = exec("tfswitch", Vec::new())?;
            exec("terraform", args)?;
        }
        _ => {
            exec(&tfexe, args)?;
        }
    }

    Ok(())
}
