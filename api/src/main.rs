mod contracts;

use config::Config;
use contracts::registry::Registry;
use ethers_core::types::Address;
use ethers_providers::{Http, Provider};
use serde_derive::Deserialize;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    rpc_url: String,
    registry_address: String,
}

fn main() {
    let settings = Config::builder()
        .add_source(config::File::with_name("settings.toml"))
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    println!("{:?}", &settings);

    let client = Arc::new(Provider::<Http>::try_from(settings.rpc_url).unwrap());
    let address: Address = settings.registry_address.parse().unwrap();
    let registry = Registry::new(address, client);
    let all_vaults = registry.get_all_va_vaults();
    println!("{:?}", all_vaults);
}
