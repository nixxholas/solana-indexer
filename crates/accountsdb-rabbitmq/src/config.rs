use std::collections::HashSet;

use serde::Deserialize;

use crate::{
    prelude::*,
    selectors::AccountSelector,
};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    accounts_amqp: Amqp,
    blocks_amqp: Amqp,
    tx_amqp: Amqp,

    jobs: Jobs,

    #[serde(default)]
    metrics: Metrics,

    #[serde(default)]
    accounts: Accounts,
}

#[serde_with::serde_as]
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Amqp {
    pub address: String,

    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub network: indexer_rabbitmq::accountsdb::Network,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Jobs {
    pub limit: usize,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metrics {
    pub config: Option<String>,
}

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Accounts {
    #[serde(default)]
    pub owners: HashSet<String>,

    /// Filter for changing how to interpret the `is_startup` flag.
    ///
    /// This option has three states:
    ///  - `None`: Ignore the `is_startup` flag and send all updates.
    ///  - `Some(true)`: Only send updates when `is_startup` is `true`.
    ///  - `Some(false)`: Only send updates when `is_startup` is `false`.
    #[serde(default)]
    pub startup: Option<bool>,
}

impl Config {
    pub fn read(path: &str) -> Result<Self> {
        let f = std::fs::File::open(path).context("Failed to open config file")?;
        let cfg = serde_json::from_reader(f).context("Failed to parse config file")?;

        Ok(cfg)
    }

    pub fn into_parts(self) -> Result<(Amqp, Amqp, Amqp, Jobs, Metrics, AccountSelector)> {
        let Self {
            accounts_amqp,
            blocks_amqp,
            tx_amqp,
            jobs,
            metrics,
            accounts
        } = self;

        let acct =
            AccountSelector::from_config(accounts).context("Failed to create account selector")?;

        Ok((accounts_amqp, blocks_amqp, tx_amqp, jobs, metrics, acct))
    }
}
