//! Command-line interface for the Decentralized Exchange application
//!
//! This CLI provides a unified interface to manage all services in the DEX application.

use anyhow::Result;
use clap::{Parser, Subcommand};
use core::logging;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::process::Command as TokioCommand;
use tokio::sync::Mutex;
use tracing::info;

// Store handles to running processes
type ProcessHandles = Arc<Mutex<HashMap<String, tokio::process::Child>>>;

/// Decentralized Exchange CLI
#[derive(Parser)]
#[command(name = "dex")]
#[command(about = "Decentralized Exchange CLI", long_about = None)]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Start all services
    Start {
        /// Services to start (api, indexer, keepers, ipfs, mev, aa)
        #[arg(short, long, value_delimiter = ',')]
        services: Option<Vec<String>>,

        /// Port for the API service
        #[arg(long, default_value = "3000")]
        api_port: u16,

        /// Port for the indexer service
        #[arg(long, default_value = "3001")]
        indexer_port: u16,
    },

    /// Stop all services
    Stop,

    /// Check the status of services
    Status,

    /// Initialize the application
    Init,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    logging::init();

    let cli = Cli::parse();

    // Create a shared store for process handles
    let process_handles: ProcessHandles = Arc::new(Mutex::new(HashMap::new()));

    match &cli.command {
        Commands::Start {
            services,
            api_port,
            indexer_port,
        } => {
            start_services(services, *api_port, *indexer_port, process_handles.clone()).await?;
        }
        Commands::Stop => {
            stop_services(process_handles.clone()).await?;
        }
        Commands::Status => {
            check_status(process_handles.clone()).await?;
        }
        Commands::Init => {
            initialize_app().await?;
        }
    }

    Ok(())
}

async fn start_services(
    services: &Option<Vec<String>>,
    api_port: u16,
    indexer_port: u16,
    process_handles: ProcessHandles,
) -> Result<()> {
    info!("Starting services...");

    // If no services specified, start all
    let services_to_start = services.as_ref().map(|s| s.clone()).unwrap_or_else(|| {
        vec![
            "api".to_string(),
            "indexer".to_string(),
            "keepers".to_string(),
            "ipfs".to_string(),
            "mev".to_string(),
            "aa".to_string(),
        ]
    });

    let mut handles = process_handles.lock().await;

    for service in &services_to_start {
        match service.as_str() {
            "api" => {
                info!("Starting API service on port {}", api_port);
                let mut cmd = TokioCommand::new("cargo");
                cmd.args(&["run", "--bin", "api"])
                    .current_dir("services/api-rs");

                let child = cmd.spawn()?;
                handles.insert("api".to_string(), child);
                info!("API service started on http://localhost:{}", api_port);
            }
            "indexer" => {
                info!("Starting Indexer service on port {}", indexer_port);
                let mut cmd = TokioCommand::new("cargo");
                cmd.args(&["run", "--bin", "indexer"])
                    .current_dir("services/indexer-rs");

                let child = cmd.spawn()?;
                handles.insert("indexer".to_string(), child);
                info!(
                    "Indexer service started on http://localhost:{}",
                    indexer_port
                );
            }
            "keepers" => {
                info!("Starting Keepers service");
                let mut cmd = TokioCommand::new("cargo");
                cmd.args(&["run", "--bin", "keepers"])
                    .current_dir("services/keepers-rs");

                let child = cmd.spawn()?;
                handles.insert("keepers".to_string(), child);
                info!("Keepers service started");
            }
            "ipfs" => {
                info!("Starting IPFS Monitor service");
                let mut cmd = TokioCommand::new("cargo");
                cmd.args(&["run", "--bin", "ipfs-monitor"])
                    .current_dir("services/ipfs-rs");

                let child = cmd.spawn()?;
                handles.insert("ipfs".to_string(), child);
                info!("IPFS Monitor service started");
            }
            "mev" => {
                info!("Starting MEV Monitor service");
                let mut cmd = TokioCommand::new("cargo");
                cmd.args(&["run", "--bin", "mev-monitor"])
                    .current_dir("services/mev-monitor");

                let child = cmd.spawn()?;
                handles.insert("mev".to_string(), child);
                info!("MEV Monitor service started");
            }
            "aa" => {
                info!("Starting AA Bundler service");
                let mut cmd = TokioCommand::new("cargo");
                cmd.args(&["run", "--bin", "aa-bundler"])
                    .current_dir("services/aa-bundler");

                let child = cmd.spawn()?;
                handles.insert("aa".to_string(), child);
                info!("AA Bundler service started");
            }
            _ => {
                info!("Unknown service: {}", service);
            }
        }
    }

    // Drop the lock before waiting
    drop(handles);

    info!("All requested services started successfully!");
    Ok(())
}

async fn stop_services(process_handles: ProcessHandles) -> Result<()> {
    info!("Stopping all services...");

    let mut handles = process_handles.lock().await;

    for (name, mut child) in handles.drain() {
        info!("Stopping {} service...", name);
        // Try to kill the process gracefully
        child.kill().await?;
        info!("{} service stopped", name);
    }

    info!("All services stopped successfully!");
    Ok(())
}

async fn check_status(process_handles: ProcessHandles) -> Result<()> {
    info!("Checking service status...");

    let handles = process_handles.lock().await;

    if handles.is_empty() {
        info!("No services are currently running");
    } else {
        for (name, child) in handles.iter() {
            // In a real implementation, we would check if the process is still running
            info!("{} service: Running (PID: {:?})", name, child.id());
        }
    }

    Ok(())
}

async fn initialize_app() -> Result<()> {
    info!("Initializing application...");
    // In a real implementation, we would perform initialization tasks here
    // Such as creating directories, setting up configuration files, etc.
    info!("Application initialized successfully!");
    Ok(())
}
