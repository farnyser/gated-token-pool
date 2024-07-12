use anchor_client::solana_sdk::pubkey::Pubkey;
use anyhow::Result;
use std::fmt::{Debug, Formatter};
use std::str::FromStr;

pub struct PubkeyCli(pub Pubkey);

impl FromStr for PubkeyCli {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<PubkeyCli> {
        Ok(PubkeyCli(Pubkey::from_str(s)?))
    }
}

impl Debug for PubkeyCli {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl Clone for PubkeyCli {
    fn clone(&self) -> Self {
        PubkeyCli(self.0)
    }
}
