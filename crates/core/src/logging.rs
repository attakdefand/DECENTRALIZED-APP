//! Logging utilities for the decentralized application
//!
//! This module provides centralized logging configuration using tracing.

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

/// Initialize logging for the application
pub fn init() {
    // Initialize tracing subscriber with environment filter
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .with(fmt::layer())
        .init();
}
