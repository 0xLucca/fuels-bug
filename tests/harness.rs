mod utils;
use chrono::Duration;
use fuels::prelude::*;
use utils::{create_provider_and_get_wallets, deploy_contract, tai_64_from_unix};

#[tokio::test]
async fn should_not_revert_when_current_time_is_within_window() {
    let wallets = create_provider_and_get_wallets(1).await;
    let provider = wallets[0].provider().unwrap();
    let (_contract_id, contract_caller) = deploy_contract(&wallets[0]).await.unwrap();

    let current_time = provider.latest_block_time().await.unwrap().unwrap();
    let tai_64 = tai_64_from_unix(current_time.timestamp());

    // Set window to [current time, current time + 100]
    let _ = contract_caller
        .contract
        .methods().set_time_window(tai_64, tai_64+100).call().await;

    let params = CallParameters::default().with_asset_id(AssetId::zeroed()).with_amount(1);

    // If we execute the action now, it should work because the current time is within the window
    let result = contract_caller
        .contract
        .methods().execute_some_action().call_params(params).unwrap().call().await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn should_revert_when_current_time_is_outside_window() {
    let wallets = create_provider_and_get_wallets(1).await;
    let provider = wallets[0].provider().unwrap();
    let (_contract_id, contract_caller) = deploy_contract(&wallets[0]).await.unwrap();

    let current_time = provider.latest_block_time().await.unwrap().unwrap();
    let tai_64 = tai_64_from_unix(current_time.timestamp());

    // Set window to [current time+500, current time + 2000]
    let _ = contract_caller
        .contract
        .methods().set_time_window(tai_64+500, tai_64+2000).call().await;

    let params = CallParameters::default().with_asset_id(AssetId::zeroed()).with_amount(1);

    // If we execute the action now, it should revert because the current time is outside execution window
    let result = contract_caller
        .contract
        .methods().execute_some_action().call_params(params).unwrap().call().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn bug_occurs_when_time_is_incremented() {
    let wallets = create_provider_and_get_wallets(1).await;
    let provider = wallets[0].provider().unwrap();
    let (_contract_id, contract_caller) = deploy_contract(&wallets[0]).await.unwrap();

    let current_time = provider.latest_block_time().await.unwrap().unwrap();
    let tai_64 = tai_64_from_unix(current_time.timestamp());

    // Set window to [current time+500, current time + 2000]
    let _ = contract_caller
        .contract
        .methods().set_time_window(tai_64+500, tai_64+2000).call().await;

    let params = CallParameters::default().with_asset_id(AssetId::zeroed()).with_amount(1);

    // If we execute the action now, it should revert because the current time is outside execution window
    let result = contract_caller
        .contract
        .methods().execute_some_action().call_params(params.clone()).unwrap().call().await;

    assert!(result.is_err());

    println!("Current time: {:?}", current_time);
    // Now we produce blocks and increment the time
    let new_time = current_time.checked_add_signed(Duration::seconds(600)).unwrap();
    let _ = provider.produce_blocks(10, Some(new_time)).await.unwrap();

    let new_current_time = provider.latest_block_time().await.unwrap().unwrap();
    println!("New current time: {:?}", new_current_time);

    // Now that the time has passed, we should be able to execute the action
    let result = contract_caller
        .contract
        .methods().execute_some_action().call_params(params).unwrap().call().await;

    // But it reverts with OutOfGas error
    println!("{:?}", result);
    assert!(result.is_ok());
}


