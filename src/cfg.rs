use {
    super::err::*,
    clap::Parser,
    std::{
        ffi::OsStr,
        io::Read,
        process::{Command, Stdio},
        thread::sleep,
        time::Duration,
    },
};

const YTDLP: &'static str = "yt-dlp";
const FFMPEG: &'static str = "ffmpeg";
const DELAY: Duration = Duration::from_millis(16);

pub fn download(url: &str) -> Result<()> {
    let mut child = Command::new(YTDLP)
        .args([
            "--all-subs",
            "--write-subs",
            "--convert-subs",
            "ass",
            "--embed-metadata",
            "--add-header",
            "Referer:https://www.crunchyroll.com/",
            url,
        ])
        .stderr(Stdio::piped())
        .spawn()?;

    while let Ok(None) = child.try_wait() {
        if let Some(ref mut stderr) = child.stderr {
            let mut buf = String::new();
            let bytes = stderr.read_to_string(&mut buf)?;

            if bytes > 0 && buf.starts_with("ERROR") {
                return Err(misc::Error::FailedDownload(
                    buf.split_once(' ')
                        .ok_or(misc::Error::Unexpected)?
                        .1
                        .trim()
                        .to_string(),
                )
                .into());
            }
        }
        sleep(DELAY)
    }
    child
        .wait()?
        .success()
        .then_some(())
        .ok_or(misc::Error::Unexpected.into())
}

fn test_cmd<S: AsRef<OsStr>, I: IntoIterator<Item = S>, E: Into<Error>>(
    prog: S,
    args: I,
    err: E,
) -> Result<()> {
    Command::new(prog)
        .args(args)
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()?
        .success()
        .then_some(())
        .ok_or(err.into())
}

pub fn requirements() -> Result<()> {
    test_cmd(YTDLP, ["--version"], misc::Prerequisite::YtDlP)?;
    test_cmd(FFMPEG, ["-version"], misc::Prerequisite::FFmpeg)?;
    Ok(println!("Everything is installed."))
}

#[derive(Debug, Parser)]
#[command(about, author, version)]
pub struct Config {
    #[arg(default_value = None)]
    url: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self::parse()
    }

    pub fn url(&self) -> Option<&str> {
        self.url.as_deref()
    }
}
