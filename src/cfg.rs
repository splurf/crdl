use {
    super::err::*,
    clap::Parser,
    std::{
        io::ErrorKind,
        process::{Command, Stdio},
    },
};

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct Config {
    #[arg(default_value = None)]
    url: Option<String>,

    #[arg(num_args = 1.., allow_hyphen_values = true, hide = true)]
    other_flags: Vec<String>,
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }

    pub fn other_flags(&self) -> &[String] {
        self.other_flags.as_slice()
    }
}

/** Determine if the provided program exists */
fn test_cmd(program: &'static str, p: misc::Program) -> Result<()> {
    let mut cmd = Command::new(program);

    #[cfg(target_os = "linux")]
    cmd.arg("-c");

    cmd.arg(if cfg!(windows) {
        p.to_string()
    } else {
        format!("command -v {}", program)
    })
    .stdout(Stdio::null())
    .stderr(Stdio::null())
    .status()
    .map_err(|e| -> Error {
        if let ErrorKind::NotFound = e.kind() {
            e.into()
        } else {
            p.into()
        }
    })?
    .success()
    .then_some(())
    .ok_or(p.into())
}

pub fn requirements() -> Result<()> {
    const PROGRAM: &'static str = if cfg!(windows) { "where" } else { "sh" };

    for p in [misc::Program::YtDlP, misc::Program::FFmpeg] {
        test_cmd(PROGRAM, p)?;
    }
    Ok(println!("Everything is installed."))
}
