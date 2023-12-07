mod cfg;
mod err;
mod util;

use {cfg::*, err::Result, util::*};

fn main() -> Result<()> {
    let cfg = Config::new();

    if let Some(url) = cfg.url() {
        download(url, cfg.other_flags())
    } else {
        requirements()
    }
}
