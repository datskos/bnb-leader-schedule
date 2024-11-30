## BNB chain Leader Schedule

### How it works

Every 200 blocks, 21 validators are selected and recorded in the block header. This is called a validator snapshot.
Starting 44 blocks after the snapshot, these validators take turns proposing blocks. Each validator's turn lasts 4
blocks.

This is a simple utility to predict the validator for a block.

#### Prediction Range:

The tool can predict validators for:

- Minimum: 44 blocks ahead
- Maximum: 243 blocks ahead

The exact prediction range depends on the current block's position relative to the epoch boundary.

### Usage

```shell
$ cargo build --release
$ ./target/release/bnb-schedule --start 44448696 --end 44448730
```

![example](./static/leader-schedule-example.png)
