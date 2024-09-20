use gadget_sdk as sdk;
use gadget_sdk::executor::process::manager::GadgetProcessManager;
use std::convert::Infallible;
use serde::{Serialize, Deserialize};

use color_eyre::{eyre::OptionExt, Result};
use sdk::{
    events_watcher::tangle::TangleEventsWatcher, events_watcher::SubstrateEventWatcher,
    keystore::backend::GenericKeyStore, keystore::Backend, tangle_subxt::*,
};

pub mod runner;
#[derive(Serialize, Deserialize)]
pub struct ConfigUpdate {
    key: String,
    value: String,
}

/// Runs a Gaia node and returns the outputs of each step along with the public URL.
#[sdk::job(id = 1, params(var), result(_), verifier(evm = "RunGaiaNodeBlueprint"))]
pub async fn run_gaia_node_job(var: Vec<u8>) -> Result<String, Infallible> {
    let (_, outputs) = runner::run_gaia_node().await?;
    Ok(serde_json::to_string(&outputs)?)
}

/// Stops the Gaia node using the GadgetProcessManager.
#[sdk::job(id = 2, params(var), result(_), verifier(evm = "StopGaiaNodeBlueprint"))]
pub async fn stop_gaia_node_job(var: Vec<u8>) -> Result<String, Infallible> {
    let mut manager = GadgetProcessManager::new();
    let (_, outputs) = runner::stop_gaia_node(&mut manager).await?;
    Ok(serde_json::to_string(&outputs)?)
}

/// Upgrades the Gaia node.
#[sdk::job(id = 3, params(var), result(_), verifier(evm = "UpgradeGaiaNodeBlueprint"))]
pub async fn upgrade_gaia_node_job(var: Vec<u8>) -> Result<String, Infallible> {
    let mut manager = GadgetProcessManager::new();
    let (_, outputs) = runner::upgrade_gaia_node(&mut manager).await?;
    Ok(serde_json::to_string(&outputs)?)
}

/// Updates the Gaia node configuration and restarts the node.
#[sdk::job(id = 4, params(config_updates), result(_), verifier(evm = "UpdateGaiaConfigBlueprint"))]
pub async fn update_gaia_config_job(config_updates: String) -> Result<String, Infallible> {
    let mut manager = GadgetProcessManager::new();
    let config_updates: Vec<ConfigUpdate> = serde_json::from_str(&config_updates)?;
    let config_updates: Vec<(&str, &str)> = config_updates.iter().map(|update| (update.key.as_str(), update.value.as_str())).collect();
    let (_, outputs) = runner::update_gaia_config(&mut manager, &config_updates).await?;
    Ok(serde_json::to_string(&outputs)?)
}




#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // Initialize the logger
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    tracing_subscriber::fmt()
        .compact()
        .with_target(true)
        .with_env_filter(env_filter)
        .init();

    // Initialize the environment
    let env = sdk::env::load()?;
    let keystore = env.keystore()?;
    let signer = extract_signer_from_keystore(&keystore)?;
    let client = subxt::OnlineClient::from_url(&env.tangle_rpc_endpoint).await?;

    // Create the event handler from the job
    let say_hello_job = SayHelloEventHandler {
        service_id: env.service_id,
        signer,
    };

    tracing::info!("Starting the event watcher ...");

    SubstrateEventWatcher::run(
        &TangleEventsWatcher,
        client,
        // Add more handler here if we have more functions.
        vec![Box::new(say_hello_job)],
    )
    .await?;

    Ok(())
}

fn extract_signer_from_keystore(
    keystore: &GenericKeyStore,
) -> Result<subxt_signer::sr25519::Keypair> {
    let sr25519_pubkey = keystore
        .iter_sr25519()
        .next()
        .ok_or_eyre("No sr25519 keys found in the keystore")?;
    let sr25519_secret = keystore
        .expose_sr25519_secret(&sr25519_pubkey)?
        .ok_or_eyre("No sr25519 secret found in the keystore")?;

    let mut seed = [0u8; 32];
    seed.copy_from_slice(&sr25519_secret.to_bytes()[0..32]);
    subxt_signer::sr25519::Keypair::from_secret_key(seed).map_err(Into::into)
}
