use ic_agent::Agent;

use crate::{CanisterSetup, PocketIcTestEnv};

/// Creates a new IC agent connected to the PocketIC HTTP endpoint.
///
/// The test environment must be in live mode (call `env.live(true).await` first).
///
/// # Panics
///
/// Panics if the environment is not in live mode.
pub async fn init_new_agent<S>(env: &PocketIcTestEnv<S>) -> Agent
where
    S: CanisterSetup,
{
    let endpoint = env.endpoint().expect("context must be in live mode");

    let agent = Agent::builder()
        .with_url(endpoint)
        .build()
        .expect("Failed to create agent");

    agent
        .fetch_root_key()
        .await
        .expect("Failed to fetch root key");

    agent
}
