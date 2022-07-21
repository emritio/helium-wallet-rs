use crate::{cmd::*, result::Result};

mod add;
mod assert;
mod list;
mod transfer;

#[derive(Debug, StructOpt)]
/// Display list of hotspots associated with wallet
/// or transfer a hotspot to another wallet
pub enum Cmd {
    Add(add::Cmd),
    Assert(assert::Cmd),
    List(list::Cmd),
    Transfer(Box<transfer::Cmd>),
}

impl Cmd {
    pub fn get_add_cmd(txn: &str, commit: bool) -> Result<Self> {
        let cmd = add::Cmd::new(txn, commit)?;
        Ok(Cmd::Add(cmd))
    }
}

impl Cmd {
    pub async fn run(self, opts: Opts) -> Result {
        match self {
            Self::Add(cmd) => cmd.run(opts).await,
            Self::Assert(cmd) => cmd.run(opts).await,
            Self::List(cmd) => cmd.run(opts).await,
            Self::Transfer(cmd) => cmd.run(opts).await,
        }
    }
}
