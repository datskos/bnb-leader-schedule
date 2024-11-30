use crate::constants::{ACTIVATION_OFFSET, EPOCH};

pub fn snapshot_block_for_block(bn: u64) -> u64 {
    let current_epoch = bn / EPOCH;
    let block_in_epoch = bn % EPOCH;

    if block_in_epoch < ACTIVATION_OFFSET {
        (current_epoch - 1) * EPOCH
    } else {
        current_epoch * EPOCH
    }
}
