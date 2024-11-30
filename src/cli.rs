use crate::constants::{ACTIVATION_OFFSET, BOHR_START, EPOCH};
use clap::Parser;
use eyre::eyre;
use std::ops::RangeInclusive;

#[derive(Parser)]
#[command(name = "validator-schedule")]
#[command(about = "Print validator schedule for block range")]
pub struct Cli {
    #[arg(long, help = "Start block")]
    start: u64,

    #[arg(long, help = "End block")]
    end: u64,
}

impl Cli {
    pub fn get_range(&self, bn: u64) -> eyre::Result<RangeInclusive<u64>> {
        let latest_epoch_start = bn - (bn % EPOCH);

        if self.start > self.end {
            return Err(eyre!("start cannot be after end"));
        } else if self.start < BOHR_START {
            return Err(eyre!("only care about blocks after bohr hardfork from 2024-09"));
        } else if self.end > latest_epoch_start + EPOCH + ACTIVATION_OFFSET - 1 {
            return Err(eyre!("too far into the future, sir"));
        }

        Ok(self.start..=self.end)
    }
}
