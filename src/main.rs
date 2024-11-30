mod cli;
mod constants;
mod utils;
mod validator_mapping;
mod validator_snapshot;

use crate::cli::Cli;
use crate::constants::{RPC_URL, VALIDATOR_CONF};
use crate::utils::snapshot_block_for_block;
use crate::validator_mapping::validator_names;
use crate::validator_snapshot::ValidatorSnapshot;
use alloy_network::primitives::BlockTransactionsKind;
use alloy_provider::{Provider, ProviderBuilder};
use clap::Parser;
use comfy_table::presets::ASCII_BORDERS_ONLY;
use comfy_table::{Cell, Color, Table};
use eyre::OptionExt;
use std::collections::{HashMap, HashSet};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let provider = ProviderBuilder::new().on_builtin(RPC_URL).await?;
    let bn = provider.get_block_number().await?;
    let cli = Cli::parse();
    let rng = cli.get_range(bn)?;

    let mut snapshot_cache: HashMap<u64, ValidatorSnapshot> = HashMap::new();
    let required_snapshots: HashSet<u64> = rng.clone().map(snapshot_block_for_block).collect();
    for snapshot_block in required_snapshots {
        let block = provider
            .get_block(snapshot_block.into(), BlockTransactionsKind::Hashes)
            .await?
            .ok_or_eyre("block not found")?;
        let snapshot: ValidatorSnapshot = block.header.extra_data.clone().try_into()?;
        snapshot_cache.insert(snapshot_block, snapshot);
    }
    let names = validator_names(VALIDATOR_CONF)?;

    let mut table = Table::new();
    table.load_preset(ASCII_BORDERS_ONLY).set_header(vec![
        Cell::new("Block").fg(Color::Green),
        Cell::new("Validator").fg(Color::Green),
        Cell::new("Name").fg(Color::Green),
    ]);

    for block in rng {
        let snapshot_block = snapshot_block_for_block(block);
        let snapshot = snapshot_cache.get(&snapshot_block).unwrap();
        let validator = snapshot.validator_for_block(block);
        let name = names.get(&validator).map(String::as_str).unwrap_or("--");

        table.add_row(vec![
            Cell::new(block).fg(Color::White),
            Cell::new(format!("{:?}", validator)).fg(Color::Cyan),
            Cell::new(name).fg(Color::Yellow),
        ]);
    }
    println!("{}", table);

    Ok(())
}
