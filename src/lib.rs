use color_eyre::Result;
use gadget_sdk::executor::process::manager::GadgetProcessManager;
use gadget_sdk::{self as sdk, config::ContextConfig};
use sdk::{
    events_watcher::substrate::SubstrateEventWatcher, events_watcher::tangle::TangleEventsWatcher,
    tangle_subxt::*,
};
use serde::{Deserialize, Serialize};
use std::convert::Infallible;
use structopt::StructOpt;

pub mod runner;

#[derive(Serialize, Deserialize)]
pub struct ConfigUpdate {
    key: String,
    value: String,
}

/// Runs a Gaia node and returns the outputs of each step along with the public URL.
#[sdk::job(
    id = 1,
    params(data),
    result(_),
    verifier(evm = "GaiaAiAgentBlueprint")
)]
pub async fn run_gaia_node_job(data: Vec<u8>) -> Result<String, Infallible> {
    let mut manager = GadgetProcessManager::new();
    let (_, outputs) = runner::run_gaia_node(&mut manager).await.unwrap();
    Ok(serde_json::to_string(&outputs).unwrap())
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
    let (_, outputs) = runner::stop_gaia_node(&mut manager).await.unwrap();
    Ok(serde_json::to_string(&outputs).unwrap())
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
    let (_, outputs) = runner::upgrade_gaia_node(&mut manager).await.unwrap();
    Ok(serde_json::to_string(&outputs).unwrap())
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
    let config_updates: Vec<ConfigUpdate> = serde_json::from_str(&config_updates).unwrap();
    let config_updates: Vec<(&str, &str)> = config_updates
        .iter()
        .map(|update| (update.key.as_str(), update.value.as_str()))
        .collect();
    let (_, outputs) = runner::update_gaia_config(&mut manager, &config_updates)
        .await
        .unwrap();
    Ok(serde_json::to_string(&outputs).unwrap())
}
