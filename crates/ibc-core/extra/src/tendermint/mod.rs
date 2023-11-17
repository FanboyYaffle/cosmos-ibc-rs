//! Provides convenience implementations for Tendermint-based hosts

pub mod upgrade_proposal;

mod validate_self_client;
pub use validate_self_client::ValidateSelfClientContext;

/// Re-exports necessary proto types for implementing the tendermint client
/// upgradeability feature.
pub mod proto {
    pub use ibc_proto::cosmos::upgrade::*;
}

/// ABCI store/query path for the IBC sub-store
pub const IBC_QUERY_PATH: &str = "store/ibc/key";

/// ABCI store/query path for the upgrade sub-store
pub const SDK_UPGRADE_QUERY_PATH: &str = "store/upgrade/key";
