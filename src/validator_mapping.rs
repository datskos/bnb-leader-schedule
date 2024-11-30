use alloy_primitives::private::serde::Deserialize;
use alloy_primitives::Address;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Deserialize)]
struct ValidatorInfo {
    name: String,
    address: String,
}

pub fn validator_names(path: &str) -> eyre::Result<HashMap<Address, String>> {
    let content = std::fs::read_to_string(path)?;
    let validators: Vec<ValidatorInfo> = serde_json::from_str(&content)?;

    validators
        .into_iter()
        .map(|v| {
            let address = Address::from_str(&v.address[2..])
                .map_err(|e| eyre::eyre!("Invalid address: {}", e))?;
            Ok((address, v.name))
        })
        .collect()
}
