use std::io;
use std::process::Command;

pub fn output(command: &str) -> Result<(), io::Error> {
    let status = Command::new("bash").arg("-c").arg(command).status()?;

    if status.success() {
        Ok(())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Command exited with code {:?}", status.code()),
        ))
    }
}

pub fn output_access(cmd: &str) -> Result<String, io::Error> {
    let res = Command::new("bash").arg("-c").arg(cmd).output()?;

    if res.status.success() {
        Ok(String::from_utf8_lossy(&res.stdout).to_string())
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            String::from_utf8_lossy(&res.stderr).to_string(),
        ))
    }
}
