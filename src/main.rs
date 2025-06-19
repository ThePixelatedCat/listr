mod cli;
mod lists;

use anyhow::{Context, Result};
use lists::Lists;

fn main() -> Result<()> {
    let lists = Lists::open_lists().context("Failed to open lists")?;
    cli::super_handler(lists)?;

    Ok(())
}
