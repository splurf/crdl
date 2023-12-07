use {
    crate::{err::misc, Result},
    clap::Parser,
    cmd_exists::*,
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

pub fn requirements() -> Result<()> {
    for p in [misc::Program::YtDlP, misc::Program::FFmpeg] {
        cmd_exists(p)?;
    }
    Ok(println!("Everything is installed."))
}
