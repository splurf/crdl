pub type Result<T, E = Error> = std::result::Result<T, E>;

pub mod misc {
    #[derive(Clone, Copy)]
    pub enum Program {
        YtDlP,
        FFmpeg,
    }

    impl AsRef<str> for Program {
        fn as_ref(&self) -> &str {
            match self {
                Self::YtDlP => "yt-dlp",
                Self::FFmpeg => "ffmpeg",
            }
        }
    }

    impl AsRef<std::ffi::OsStr> for Program {
        fn as_ref(&self) -> &std::ffi::OsStr {
            std::ffi::OsStr::new(AsRef::<str>::as_ref(self))
        }
    }

    impl std::fmt::Display for Program {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.as_ref())
        }
    }

    #[derive(Clone)]
    pub enum Error {
        Prerequisite(Program),
        FailedDownload(String),
        Unexpected,
    }

    impl std::fmt::Display for Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&match self {
                Error::Prerequisite(p) => format!("'{}' not found", p),
                Error::FailedDownload(e) => e.to_string(),
                Self::Unexpected => "An unexpected error has occurred".to_string(),
            })
        }
    }
}

pub enum Error {
    IO(std::io::Error),
    Misc(misc::Error),
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl From<misc::Error> for Error {
    fn from(value: misc::Error) -> Self {
        Self::Misc(value)
    }
}

impl From<misc::Program> for Error {
    fn from(value: misc::Program) -> Self {
        Self::Misc(misc::Error::Prerequisite(value))
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self, f)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&match self {
            Error::IO(e) => e.to_string(),
            Error::Misc(e) => e.to_string(),
        })
    }
}
