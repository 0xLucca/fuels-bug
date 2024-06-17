use fuels::{accounts::wallet::WalletUnlocked, macros::abigen, programs::contract::{Contract, LoadConfiguration}, test_helpers::{launch_custom_provider_and_get_wallets, WalletsConfig}, types::{bech32::Bech32ContractId, transaction::TxPolicies}};
use fuels::prelude::Error;
abigen!(
    Contract(
        name = "MyContract",
        abi = "out/debug/timestamp-testing-abi.json"
    )
);

pub async fn create_provider_and_get_wallets(num_wallets: u64) -> Vec<WalletUnlocked> {
    launch_custom_provider_and_get_wallets(
        WalletsConfig::new(
            Some(num_wallets),   /* "num_wallets" wallets */
            Some(1),             /* Single coin (UTXO) */
            Some(1_000_000_000), /* Amount per coin */
        ),
        //Some(config),
        None,
        None,
    )
    .await
    .unwrap()
}

pub struct ContractCaller {
    pub contract: MyContract<WalletUnlocked>,
    pub wallet: WalletUnlocked,
}

pub async fn deploy_contract(
    deployer: &WalletUnlocked,
) -> Result<(Bech32ContractId, ContractCaller), Error> {
    // Deploy the contract
    let contract_id = Contract::load_from(
        "./out/debug/timestamp-testing.bin",
        LoadConfiguration::default(),
    )
    .unwrap()
    .deploy(deployer, TxPolicies::default())
    .await
    .unwrap();

    // Create a caller instance
    let deployer = ContractCaller {
        contract: MyContract::new(contract_id.clone(), deployer.clone()),
        wallet: deployer.clone(),
    };

    Ok((contract_id, deployer))
}

/// Convert Unix timestamp to `TAI64`.
pub fn tai_64_from_unix(secs: i64) -> u64 {
    (secs + 10 + (1 << 62)) as u64
}

/// Convert `TAI64` to unix timestamp.
pub fn tai_64_to_unix(tai_64: u64) -> i64 {
    (tai_64 as i64) - (10 + (1 << 62))
}