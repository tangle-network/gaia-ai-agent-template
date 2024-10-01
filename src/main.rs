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


pub use gaia_ai_agent_template as blueprint;

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
    let config = ContextConfig::from_args();
    let env = sdk::config::load(config)?;
    let signer = env.first_sr25519_signer()?;
    let client = subxt::OnlineClient::from_url(&env.rpc_endpoint).await?;

    if env.should_run_registration() {
        todo!();
    }

    let service_id = env.service_id.expect("should exist");
    tracing::info!("Starting the event watcher ...");

    SubstrateEventWatcher::run(
        &TangleEventsWatcher {
            span: env.span.clone(),
        },
        client,
        vec![],
    )
    .await
    .unwrap();

    Ok(())
}
