pub mod avail_subxt_config;
pub use avail_subxt_config::{avail, AvailConfig};

pub mod home;

mod nomad_core;
pub use crate::nomad_core::*;

mod nomad_base;
pub use nomad_base::*;

#[macro_use]
mod macros;
pub use macros::*;

pub type SubstrateSigner<T> = dyn subxt::tx::Signer<T> + Send + Sync;
