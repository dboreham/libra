//! `version` subcommand

#![allow(clippy::never_loop)]

use crate::{block::build_block, config, keygen, account::UserConfigs};
use abscissa_core::{Command, Options, Runnable};
use std::path::PathBuf;

/// `version` subcommand
#[derive(Command, Debug, Default, Options)]
pub struct CreateCmd {
    #[options(help = "don't generate keys")]
    skip_keys: bool,
    #[options(help = "create a validator account, instead of user account")]
    val: bool,
    #[options(help = "path to write account manifest")]
    path: PathBuf,
}

impl Runnable for CreateCmd {
    /// Print version message
    fn run(&self) {
        let mut miner_configs = config::MinerConfig::default();
        let (authkey, account) = keygen::keygen();
        miner_configs.profile.auth_key = authkey.to_string();
        miner_configs.profile.account = account;
        let block = build_block::mine_genesis(&miner_configs);
        let configs = UserConfigs::new(block);
        configs.create_user_manifest(&mut self.path.clone());
    }
}
