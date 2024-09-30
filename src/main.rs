use gadget_sdk as sdk;
use gadget_sdk::executor::process::manager::GadgetProcessManager;
use serde::{Deserialize, Serialize};
use std::convert::Infallible;

use color_eyre::{eyre::OptionExt, Result};
use sdk::{
    events_watcher::tangle::TangleEventsWatcher, events_watcher::substrate::SubstrateEventWatcher,
    keystore::backend::GenericKeyStore, keystore::Backend, tangle_subxt::*,
};

pub mod runner;
#[derive(Serialize, Deserialize)]
pub struct ConfigUpdate {
    key: String,
    value: String,
}

/// Runs a Gaia node and returns the outputs of each step along with the public URL.
#[sdk::job(id = 1, params(data), result(_), verifier(evm = "GaiaAiAgentBlueprint"))]
pub async fn run_gaia_node_job(data: Vec<u8>) -> Result<String, Infallible> {
    let (_, outputs) = runner::run_gaia_node().await?;
    Ok(serde_json::to_string(&outputs)?)
}

/// Stops the Gaia node using the GadgetProcessManager.
#[sdk::job(
    id = 2,
    params(data),
    result(_),
    verifier(evm = "GaiaAiAgentBlueprint")
)]
pub async fn stop_gaia_node_job(data: Vec<u8>) -> Result<String, Infallible> {
    let mut manager = GadgetProcessManager::new();
    let (_, outputs) = runner::stop_gaia_node(&mut manager).await?;
    Ok(serde_json::to_string(&outputs)?)
}

/// Upgrades the Gaia node.
#[sdk::job(
    id = 3,
    params(data),
    result(_),
    verifier(evm = "GaiaAiAgentBlueprint")
)]
pub async fn upgrade_gaia_node_job(data: Vec<u8>) -> Result<String, Infallible> {
    let mut manager = GadgetProcessManager::new();
    let (_, outputs) = runner::upgrade_gaia_node(&mut manager).await?;
    Ok(serde_json::to_string(&outputs)?)
}

/// Updates the Gaia node configuration and restarts the node.
#[sdk::job(
    id = 4,
    params(config_updates),
    result(_),
    verifier(evm = "GaiaAiAgentBlueprint")
)]
pub async fn update_gaia_config_job(config_updates: String) -> Result<String, Infallible> {
    let mut manager = GadgetProcessManager::new();
    let config_updates: Vec<ConfigUpdate> = serde_json::from_str(&config_updates)?;
    let config_updates: Vec<(&str, &str)> = config_updates
        .iter()
        .map(|update| (update.key.as_str(), update.value.as_str()))
        .collect();
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
    let signer = keystore.first_signer()?;
    let client = subxt::OnlineClient::from_url(&env.tangle_rpc_endpoint).await?;

    tracing::info!("Starting the event watcher ...");

    SubstrateEventWatcher::run(
        &TangleEventsWatcher,
        client,
        // Add more handler here if we have more functions.
        vec![],
    )
    .await?;

    Ok(())
}
