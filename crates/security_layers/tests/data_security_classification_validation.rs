//! Data Security Classification Validation Tests
//!
//! This module contains tests that validate the data classification functionality
//! as defined in the web3_protection_layers.csv file for Layer 5.

use security_layers::data_security::{
    DataClassification, ClassifiedDataAsset, DataInventory, DataClassificationManager,
};

/// Test DataClassification enum functionality
#[test]
fn test_data_classification_enum() {
    // Test as_str method
    assert_eq!(DataClassification::Public.as_str(), "public");
    assert_eq!(DataClassification::Internal.as_str(), "internal");
    assert_eq!(DataClassification::Confidential.as_str(), "confidential");
    assert_eq!(DataClassification::Restricted.as_str(), "restricted");
    
    // Test from_str method
    assert_eq!(DataClassification::from_str("public"), Some(DataClassification::Public));
    assert_eq!(DataClassification::from_str("INTERNAL"), Some(DataClassification::Internal));
    assert_eq!(DataClassification::from_str("Confidential"), Some(DataClassification::Confidential));
    assert_eq!(DataClassification::from_str("RESTRICTED"), Some(DataClassification::Restricted));
    assert_eq!(DataClassification::from_str("invalid"), None);
}

/// Test ClassifiedDataAsset creation and functionality
#[test]
fn test_classified_data_asset() {
    let mut asset = ClassifiedDataAsset::new(
        "asset-001".to_string(),
        "User Email Data".to_string(),
        DataClassification::Confidential,
        "database/users".to_string(),
        "data-owner@example.com".to_string(),
    );
    
    // Test basic properties
    assert_eq!(asset.id, "asset-001");
    assert_eq!(asset.name, "User Email Data");
    assert_eq!(asset.classification, DataClassification::Confidential);
    assert_eq!(asset.storage_location, "database/users");
    assert_eq!(asset.owner, "data-owner@example.com");
    assert!(asset.classified_at > 0);
    
    // Test metadata functionality
    asset.add_metadata("department".to_string(), "engineering".to_string());
    asset.add_metadata("pii".to_string(), "true".to_string());
    
    assert_eq!(asset.metadata.get("department"), Some(&"engineering".to_string()));
    assert_eq!(asset.metadata.get("pii"), Some(&"true".to_string()));
}

/// Test DataInventory functionality
#[test]
fn test_data_inventory() {
    let mut inventory = DataInventory::new();
    
    // Add assets with different classifications
    let asset1 = ClassifiedDataAsset::new(
        "asset-001".to_string(),
        "Public Documentation".to_string(),
        DataClassification::Public,
        "docs/public".to_string(),
        "docs-team@example.com".to_string(),
    );
    
    let asset2 = ClassifiedDataAsset::new(
        "asset-002".to_string(),
        "Internal Meeting Notes".to_string(),
        DataClassification::Internal,
        "docs/internal".to_string(),
        "hr-team@example.com".to_string(),
    );
    
    let asset3 = ClassifiedDataAsset::new(
        "asset-003".to_string(),
        "User Financial Data".to_string(),
        DataClassification::Confidential,
        "database/users".to_string(),
        "security-team@example.com".to_string(),
    );
    
    let asset4 = ClassifiedDataAsset::new(
        "asset-004".to_string(),
        "Executive Strategy Documents".to_string(),
        DataClassification::Restricted,
        "docs/executive".to_string(),
        "executive-team@example.com".to_string(),
    );
    
    inventory.add_asset(asset1);
    inventory.add_asset(asset2);
    inventory.add_asset(asset3);
    inventory.add_asset(asset4);
    
    // Test getting assets by ID
    assert!(inventory.get_asset("asset-001").is_some());
    assert!(inventory.get_asset("asset-002").is_some());
    assert!(inventory.get_asset("asset-003").is_some());
    assert!(inventory.get_asset("asset-004").is_some());
    assert!(inventory.get_asset("nonexistent").is_none());
    
    // Test getting assets by classification
    let public_assets = inventory.get_assets_by_classification(&DataClassification::Public);
    assert_eq!(public_assets.len(), 1);
    assert_eq!(public_assets[0].id, "asset-001");
    
    let internal_assets = inventory.get_assets_by_classification(&DataClassification::Internal);
    assert_eq!(internal_assets.len(), 1);
    assert_eq!(internal_assets[0].id, "asset-002");
    
    let confidential_assets = inventory.get_assets_by_classification(&DataClassification::Confidential);
    assert_eq!(confidential_assets.len(), 1);
    assert_eq!(confidential_assets[0].id, "asset-003");
    
    let restricted_assets = inventory.get_assets_by_classification(&DataClassification::Restricted);
    assert_eq!(restricted_assets.len(), 1);
    assert_eq!(restricted_assets[0].id, "asset-004");
    
    // Test getting all assets
    let all_assets = inventory.get_all_assets();
    assert_eq!(all_assets.len(), 4);
    
    // Test classification counts
    let counts = inventory.get_classification_counts();
    assert_eq!(counts.get(&DataClassification::Public), Some(&1));
    assert_eq!(counts.get(&DataClassification::Internal), Some(&1));
    assert_eq!(counts.get(&DataClassification::Confidential), Some(&1));
    assert_eq!(counts.get(&DataClassification::Restricted), Some(&1));
    
    // Test updating classification
    assert!(inventory.update_classification("asset-001", DataClassification::Confidential).is_ok());
    
    // Verify the update
    let updated_asset = inventory.get_asset("asset-001").unwrap();
    assert_eq!(updated_asset.classification, DataClassification::Confidential);
    
    // Check that the asset is now in the confidential group
    let confidential_assets = inventory.get_assets_by_classification(&DataClassification::Confidential);
    assert_eq!(confidential_assets.len(), 2);
    
    // Check that the asset is no longer in the public group
    let public_assets = inventory.get_assets_by_classification(&DataClassification::Public);
    assert_eq!(public_assets.len(), 0);
    
    // Test removing an asset
    assert!(inventory.remove_asset("asset-002").is_ok());
    assert!(inventory.get_asset("asset-002").is_none());
    
    // Verify the count decreased
    let all_assets = inventory.get_all_assets();
    assert_eq!(all_assets.len(), 3);
    
    // Test removing a non-existent asset
    assert!(inventory.remove_asset("nonexistent").is_err());
    
    // Test updating a non-existent asset
    assert!(inventory.update_classification("nonexistent", DataClassification::Public).is_err());
}

/// Test DataClassificationManager functionality
#[test]
fn test_data_classification_manager() {
    let mut manager = DataClassificationManager::new();
    
    // Test classifying an asset
    let asset = manager.classify_asset(
        "asset-001".to_string(),
        "Customer Payment Information".to_string(),
        DataClassification::Restricted,
        "payments/database".to_string(),
        "finance-team@example.com".to_string(),
    );
    
    // Verify the asset was created correctly
    assert_eq!(asset.id, "asset-001");
    assert_eq!(asset.classification, DataClassification::Restricted);
    
    // Verify the asset was added to the inventory
    let inventory = manager.get_inventory();
    assert!(inventory.get_asset("asset-001").is_some());
    
    // Test telemetry report generation
    let report = manager.generate_telemetry_report();
    assert!(report.contains("Data Inventory with Labels:"));
    assert!(report.contains("Total Assets: 1"));
    assert!(report.contains("restricted: 1"));
    assert!(report.contains("ID: asset-001"));
    assert!(report.contains("Customer Payment Information"));
    assert!(report.contains("payments/database"));
    assert!(report.contains("finance-team@example.com"));
}

/// Test the specific requirement from the CSV: "Classify data: public / internal / confidential / restricted"
#[test]
fn test_csv_requirement_classification_tiers() {
    let mut manager = DataClassificationManager::new();
    
    // Create assets for each classification tier
    manager.classify_asset(
        "public-doc-001".to_string(),
        "Public Whitepaper".to_string(),
        DataClassification::Public,
        "public/docs".to_string(),
        "marketing@example.com".to_string(),
    );
    
    manager.classify_asset(
        "internal-doc-001".to_string(),
        "Internal Process Document".to_string(),
        DataClassification::Internal,
        "internal/docs".to_string(),
        "operations@example.com".to_string(),
    );
    
    manager.classify_asset(
        "confidential-data-001".to_string(),
        "User Personal Information".to_string(),
        DataClassification::Confidential,
        "database/users".to_string(),
        "security@example.com".to_string(),
    );
    
    manager.classify_asset(
        "restricted-data-001".to_string(),
        "Executive Financial Projections".to_string(),
        DataClassification::Restricted,
        "executive/financials".to_string(),
        "cfo@example.com".to_string(),
    );
    
    // Verify all four classification tiers are implemented
    let inventory = manager.get_inventory();
    assert_eq!(inventory.get_assets_by_classification(&DataClassification::Public).len(), 1);
    assert_eq!(inventory.get_assets_by_classification(&DataClassification::Internal).len(), 1);
    assert_eq!(inventory.get_assets_by_classification(&DataClassification::Confidential).len(), 1);
    assert_eq!(inventory.get_assets_by_classification(&DataClassification::Restricted).len(), 1);
    
    // Verify the goal: "Know which data needs strong controls"
    // Public data needs minimal controls
    let public_assets = inventory.get_assets_by_classification(&DataClassification::Public);
    assert_eq!(public_assets[0].storage_location, "public/docs");
    
    // Restricted data needs strong controls
    let restricted_assets = inventory.get_assets_by_classification(&DataClassification::Restricted);
    assert_eq!(restricted_assets[0].storage_location, "executive/financials");
    
    // Verify evidence/telemetry: "Data inventory with labels"
    let report = manager.generate_telemetry_report();
    assert!(report.contains("Data Inventory with Labels:"));
    assert!(report.contains("public: 1"));
    assert!(report.contains("internal: 1"));
    assert!(report.contains("confidential: 1"));
    assert!(report.contains("restricted: 1"));
    assert!(report.contains("Public Whitepaper"));
    assert!(report.contains("Internal Process Document"));
    assert!(report.contains("User Personal Information"));
    assert!(report.contains("Executive Financial Projections"));
}

/// Integration test showing how the data classification system works with the overall security layers
#[test]
fn test_data_classification_integration() {
    let mut manager = DataClassificationManager::new();
    
    // Simulate classifying various data assets in a DEX application
    manager.classify_asset(
        "user-wallet-addresses".to_string(),
        "User Wallet Addresses".to_string(),
        DataClassification::Confidential,
        "database/users/wallets".to_string(),
        "security-team@example.com".to_string(),
    );
    
    manager.classify_asset(
        "trade-history".to_string(),
        "User Trade History".to_string(),
        DataClassification::Confidential,
        "database/trades".to_string(),
        "security-team@example.com".to_string(),
    );
    
    manager.classify_asset(
        "market-data".to_string(),
        "Public Market Data".to_string(),
        DataClassification::Public,
        "database/market".to_string(),
        "data-team@example.com".to_string(),
    );
    
    manager.classify_asset(
        "admin-credentials".to_string(),
        "Administrative Credentials".to_string(),
        DataClassification::Restricted,
        "vault/admin".to_string(),
        "security-team@example.com".to_string(),
    );
    
    // Verify the inventory
    let inventory = manager.get_inventory();
    assert_eq!(inventory.get_assets_by_classification(&DataClassification::Public).len(), 1);
    assert_eq!(inventory.get_assets_by_classification(&DataClassification::Confidential).len(), 2);
    assert_eq!(inventory.get_assets_by_classification(&DataClassification::Restricted).len(), 1);
    
    // Generate the required evidence/telemetry
    let telemetry_report = manager.generate_telemetry_report();
    println!("Telemetry Report:\n{}", telemetry_report);
    
    // Verify that we can identify which data needs strong controls
    let confidential_assets = inventory.get_assets_by_classification(&DataClassification::Confidential);
    let restricted_assets = inventory.get_assets_by_classification(&DataClassification::Restricted);
    
    // These should have strong access controls
    assert_eq!(confidential_assets.len(), 2);
    assert_eq!(restricted_assets.len(), 1);
    
    // This should have minimal controls
    let public_assets = inventory.get_assets_by_classification(&DataClassification::Public);
    assert_eq!(public_assets.len(), 1);
}