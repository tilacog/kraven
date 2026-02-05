use anyhow::Result;

use crate::config::KRAVEN_ACTIVE;

pub fn run() -> Result<()> {
    let profile =
        std::env::var(KRAVEN_ACTIVE).map_err(|_| anyhow::anyhow!("No profile active."))?;
    println!("{profile}");
    Ok(())
}
