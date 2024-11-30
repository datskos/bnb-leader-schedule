use crate::constants::{EXTRA_SEAL, EXTRA_VANITY, NUMBER_SIZE, VALIDATOR_BYTES_LEN};
use alloy_primitives::{Address, Bytes};
use eyre::Report;

#[derive(Debug, Clone)]
pub struct ValidatorSnapshot {
    validators: Vec<Address>,
    turn_length: u8,
}

impl ValidatorSnapshot {
    pub fn validator_for_block(&self, bn: u64) -> Address {
        let idx = (bn / self.turn_length as u64) % self.validators.len() as u64;
        self.validators[idx as usize]
    }
}

impl TryFrom<Bytes> for ValidatorSnapshot {
    type Error = Report;

    fn try_from(value: Bytes) -> Result<Self, Self::Error> {
        if value.len() <= EXTRA_VANITY + EXTRA_SEAL {
            return Err(eyre::eyre!("Invalid input length"));
        }

        let num_validators = value[EXTRA_VANITY] as usize;
        let start = EXTRA_VANITY + NUMBER_SIZE;
        let end = start + num_validators * VALIDATOR_BYTES_LEN;
        let extra_min_len = end + EXTRA_SEAL;

        if num_validators == 0 || value.len() < extra_min_len {
            return Err(eyre::eyre!("Invalid validator count or length"));
        }

        let validator_bytes = &value[start..end];
        let mut validators = Vec::with_capacity(num_validators);

        for chunk in validator_bytes.chunks(VALIDATOR_BYTES_LEN) {
            let address_bytes: [u8; 20] =
                chunk[..20].try_into().map_err(|_| eyre::eyre!("Invalid address bytes"))?;
            validators.push(Address::from(address_bytes));
        }

        let turn_length = if value.len() > extra_min_len { value[end] } else { 0 };

        Ok(ValidatorSnapshot { validators, turn_length })
    }
}
