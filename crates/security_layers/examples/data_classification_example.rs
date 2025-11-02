//! Example of using the Data Classification system
//!
//! This example demonstrates how to use the data classification functionality
//! to classify data assets according to sensitivity tiers.

use security_layers::data_security::{
    DataClassification, DataClassificationManager,
};

fn main() {
    println!("Data Classification Example");
    println!("========================");
    
    // Create a data classification manager
    let mut manager = DataClassificationManager::new();
    
    // Classify various data assets in a DEX application
    println!("\n1. Classifying data assets...");
    
    let public_asset = manager.classify_asset(
        "market-data-001".to_string(),
        "Public Market Data".to_string(),
        DataClassification::Public,
        "database/market".to_string(),
        "data-team@example.com".to_string(),
    );
    println!("   Classified public asset: {}", public_asset.name);
    
    let internal_asset = manager.classify_asset(
        "process-doc-001".to_string(),
        "Internal Trading Process".to_string(),
        DataClassification::Internal,
        "docs/internal".to_string(),
        "operations@example.com".to_string(),
    );
    println!("   Classified internal asset: {}", internal_asset.name);
    
    let confidential_asset = manager.classify_asset(
        "user-wallets-001".to_string(),
        "User Wallet Addresses".to_string(),
        DataClassification::Confidential,
        "database/users/wallets".to_string(),
        "security-team@example.com".to_string(),
    );
    println!("   Classified confidential asset: {}", confidential_asset.name);
    
    let restricted_asset = manager.classify_asset(
        "admin-keys-001".to_string(),
        "Administrative Private Keys".to_string(),
        DataClassification::Restricted,
        "vault/admin".to_string(),
        "security-team@example.com".to_string(),
    );
    println!("   Classified restricted asset: {}", restricted_asset.name);
    
    // Generate telemetry/evidence report
    println!("\n2. Generating telemetry report...");
    let report = manager.generate_telemetry_report();
    println!("{}", report);
    
    // Show how to access specific classification data
    println!("3. Accessing classified data by sensitivity level...");
    let inventory = manager.get_inventory();
    
    let confidential_assets = inventory.get_assets_by_classification(&DataClassification::Confidential);
    println!("   Confidential assets count: {}", confidential_assets.len());
    for asset in confidential_assets {
        println!("     - {} (stored at: {})", asset.name, asset.storage_location);
    }
    
    let restricted_assets = inventory.get_assets_by_classification(&DataClassification::Restricted);
    println!("   Restricted assets count: {}", restricted_assets.len());
    for asset in restricted_assets {
        println!("     - {} (stored at: {})", asset.name, asset.storage_location);
    }
    
    println!("\nData classification complete!");
    println!("This implementation satisfies the requirement:");
    println!("  \"Classify data: public / internal / confidential / restricted\"");
    println!("  \"Know which data needs strong controls\"");
    println!("  \"Data inventory with labels\"");
}