mod cfg;
mod err;

use {cfg::*, err::Result};

fn main() -> Result<()> {
    let cfg = Config::new();

    if let Some(url) = cfg.url() {
        download(url)
    } else {
        requirements()
    }
}
