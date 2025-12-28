// SPDX-License-Identifier: Apache-2.0

//! Audio Ninja Daemon Library
//!
//! This provides the API and engine state for testing.

pub mod api;
pub mod engine;

pub use engine::EngineState;

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct AppState {
    pub engine: Arc<RwLock<EngineState>>,
}
