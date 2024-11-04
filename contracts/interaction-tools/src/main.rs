extern crate dotenv;

use dotenv::dotenv;
use fuels::prelude::*;
use fuels::types::{AssetId, Bits256};
use rand::Rng;
use std::env;
use std::str::FromStr;

abigen!(Contract(
    name = "Pool",
    abi = "../pool/out/release/pool-abi.json"
),);

#[tokio::main]
async fn main() -> Result<()> {
    // Load .env file
    dotenv().ok();
    let phrase = env::var("MNEMONIC").expect("MNEMONIC must be set in .env");
    let provider_url = env::var("PROVIDER_URL").expect("PROVIDER must be set in .env");
    let base_asset_address =
        env::var("BASE_ASSET_ADDRESS").expect("BASE_ASSET_ADDRESS must be set in .env");

    let provider = Provider::connect(provider_url).await.unwrap();
    let wallet = WalletUnlocked::new_from_mnemonic_phrase(&phrase, Some(provider.clone())).unwrap();
    let base_asset_id = AssetId::from_str(&base_asset_address).unwrap();

    let balance = provider
        .get_asset_balance(wallet.address(), base_asset_id)
        .await?;
    if balance < 500000 {
        println!("Wallet address {}, with balance {}, can use the faucet here: https://faucet-testnet.fuel.network/?address={}", wallet.address(), balance, wallet.address());
        return Ok(());
    }

    // Random Salt to deploy a new contract on every run
    let mut salt = [0u8; 32];
    rand::thread_rng().fill(&mut salt);

    let contract_id = Contract::load_from(
        "../pool/out/release/pool.bin",
        LoadConfiguration::default().with_salt(salt),
    )?
    .deploy(&wallet, TxPolicies::default())
    .await?;
    println!("Deployed Pool contract: 0x{}", contract_id.hash);

    let contract_methods = Pool::new(contract_id.clone(), wallet.clone()).methods();

    let deposit_amount = 100;
    let call_params = CallParameters::default()
        .with_amount(deposit_amount)
        .with_asset_id(base_asset_id);
    let r = contract_methods
        .deposit(wallet.address().into())
        .call_params(call_params)?
        .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
        .call()
        .await?;
    println!(
        "Deposited {deposit_amount} Eth to LP in tx: 0x{}",
        r.tx_id.unwrap()
    );

    let lp_asset_id = contract_id.asset_id(&Bits256::zeroed());
    let lp_token_balance = wallet.get_asset_balance(&lp_asset_id).await?;
    let call_params = CallParameters::default()
        .with_amount(lp_token_balance)
        .with_asset_id(lp_asset_id);
    let r = contract_methods
        .withdraw(wallet.address().into())
        .call_params(call_params)?
        .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
        .call()
        .await?;
    println!(
        "Withdrawn {} eth in tx: 0x{}",
        lp_token_balance,
        r.tx_id.unwrap()
    );

    let deposit_amount = 100;
    let call_params = CallParameters::default()
        .with_amount(deposit_amount)
        .with_asset_id(base_asset_id);
    let r = contract_methods
        .deposit(wallet.address().into())
        .call_params(call_params)?
        .with_variable_output_policy(VariableOutputPolicy::Exactly(1))
        .call()
        .await?;
    println!(
        "Deposited {deposit_amount} Eth to LP in tx: 0x{}",
        r.tx_id.unwrap()
    );

    println!("Successfully finished mock interactions with the contract: {contract_id}",);

    Ok(())
}
