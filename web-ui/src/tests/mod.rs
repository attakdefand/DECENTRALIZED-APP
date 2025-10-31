//! Integration tests for web-ui features
//!
//! This module contains comprehensive tests for authentication, caching,
//! retry logic, throttling, WebSocket functionality, and backend integration
//! with security validation.

#[cfg(test)]
mod security_tests;

#[cfg(test)]
mod integration_tests;

#[cfg(test)]
mod component_tests;

#[cfg(test)]
mod backend_tests;

#[cfg(test)]
mod accessibility_tests;

#[cfg(test)]
mod liquidity_security_tests;

#[cfg(test)]
mod performance_tests;
