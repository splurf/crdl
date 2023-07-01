use std::fmt::{Debug, Display};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod misc {
    pub enum Prerequisite {
        YtDlP,
        FFmpeg,
    }

    impl super::Display for Prerequisite {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(match self {
                Self::YtDlP => "yt-dlp",
                Self::FFmpeg => "FFmpeg",
            })
        }
    }

    pub enum Error {
        Prerequisite(Prerequisite),
        FailedDownload(String),
        Unexpected,
    }

    impl super::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&match self {
                Error::Prerequisite(p) => format!("Missing \"{}\"", p),
                Error::FailedDownload(e) => e.clone(),
                Self::Unexpected => "An unexpected error has occurred".to_string(),
            })
        }
    }
}

pub enum Error {
    Io(std::io::Error),
    Misc(misc::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<misc::Error> for Error {
    fn from(value: misc::Error) -> Self {
        Self::Misc(value)
    }
}

impl From<misc::Prerequisite> for Error {
    fn from(value: misc::Prerequisite) -> Self {
        Self::Misc(misc::Error::Prerequisite(value))
    }
}

impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Error::Io(e) => e.to_string(),
            Error::Misc(e) => e.to_string(),
        })
    }
}
