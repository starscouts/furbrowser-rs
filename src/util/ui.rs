use std::io;
use std::io::{BufRead, Write};
use std::process::{Command, ExitStatus, Stdio};
use crate::error::{FurbrowserError, FurbrowserResult};

pub fn yesno(prompt: &str, yes: &str, no: &str) -> FurbrowserResult<bool> {
    loop {
        print!("{}[2K\r", 27 as char);
        print!("{prompt} ({yes}/{no})");
        io::stdout().flush()?;

        let stdin = io::stdin();
        let mut iterator = stdin.lock().lines();
        let line = iterator.next().ok_or(FurbrowserError::Readline)??;

        if line == yes {
            return Ok(true);
        } else if line == no {
            return Ok(false);
        }
    }
}

pub fn clear() -> FurbrowserResult<()> {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    io::stdout().flush()?;
    Ok(())
}

pub fn image(url: &str) -> FurbrowserResult<ExitStatus> {
    Ok(Command::new("bash")
        .arg("-c")
        .arg(&format!("$HOME/.iterm2/imgcat -W 100% -H 90% -u \"{}\"", url))
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?)
}