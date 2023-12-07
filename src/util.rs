use {
    super::err::*,
    std::{
        io::Read,
        process::{Command, Stdio},
        thread::sleep,
        time::Duration,
    },
};

const DELAY: Duration = Duration::from_millis(16);

pub fn download(url: &str, other_flags: &[String]) -> Result<()> {
    let mut args = vec![
        "--all-subs",
        "--write-subs",
        "--convert-subs",
        "ass",
        "--embed-metadata",
        "--add-header",
        "Referer:https://www.crunchyroll.com/",
    ];
    args.extend(other_flags.iter().map(String::as_str));
    args.push(url);

    let mut child = Command::new(misc::Program::YtDlP)
        .args(args)
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
