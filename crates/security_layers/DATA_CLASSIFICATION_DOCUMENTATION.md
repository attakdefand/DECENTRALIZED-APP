# Data Classification with Sensitivity Tiering

## Overview

This document describes the implementation of Data Classification with Sensitivity Tiering as part of Layer 5: Data Security in the Web3 Protection Layers framework. This implementation satisfies the requirements specified in the `web3_protection_layers.csv` file:

- **Component / Mechanism**: "Classify data: public / internal / confidential / restricted"
- **Goal**: "Know which data needs strong controls"
- **Evidence / Telemetry**: "Data inventory with labels"

## Implementation Details

### Data Classification Levels

The implementation provides four sensitivity tiers for data classification:

1. **Public** - Data that can be freely shared with the public
2. **Internal** - Data for internal organizational use only
3. **Confidential** - Data requiring protection due to sensitivity
4. **Restricted** - Data with strict access controls and highest protection requirements

### Core Components

#### DataClassification Enum

The `DataClassification` enum represents the four sensitivity tiers:

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DataClassification {
    Public,
    Internal,
    Confidential,
    Restricted,
}
```

#### ClassifiedDataAsset Struct

The `ClassifiedDataAsset` struct represents a data asset with its classification:

```rust
pub struct ClassifiedDataAsset {
    pub id: String,
    pub name: String,
    pub classification: DataClassification,
    pub storage_location: String,
    pub owner: String,
    pub classified_at: u64,
    pub metadata: HashMap<String, String>,
}
```

#### DataInventory Struct

The `DataInventory` struct manages all classified data assets and provides efficient lookup by classification:

```rust
pub struct DataInventory {
    assets: HashMap<String, ClassifiedDataAsset>,
    classification_index: HashMap<DataClassification, Vec<String>>,
}
```

#### DataClassificationManager

The `DataClassificationManager` provides a high-level interface for classifying data assets and generating telemetry reports:

```rust
pub struct DataClassificationManager {
    inventory: DataInventory,
}
```

## Usage Examples

### Basic Usage

```rust
use security_layers::data_security::{
    DataClassification, DataClassificationManager,
};

// Create a data classification manager
let mut manager = DataClassificationManager::new();

// Classify a data asset
let asset = manager.classify_asset(
    "asset-001".to_string(),
    "User Financial Data".to_string(),
    DataClassification::Confidential,
    "database/users".to_string(),
    "security-team@example.com".to_string(),
);

// Generate a telemetry report
let report = manager.generate_telemetry_report();
println!("{}", report);
```

### Retrieving Assets by Classification

```rust
// Get all confidential assets
let inventory = manager.get_inventory();
let confidential_assets = inventory.get_assets_by_classification(&DataClassification::Confidential);

for asset in confidential_assets {
    println!("Confidential asset: {} stored at {}", asset.name, asset.storage_location);
}
```

## Integration with Security Layers

This implementation integrates with the broader security layers framework and satisfies the validation requirements in `security_layers_validation.rs`:

```rust
// Test that validates the data classification layer from the CSV file
fn test_layer_5_data_security(layers: &[SecurityLayer]) {
    // Find the Data Classification layer
    let data_classification_layer = layers.iter().find(|l| 
        l.layer_number == 5 && 
        l.main_type == "Data Classification" && 
        l.sub_type == "Sensitivity Tiering"
    ).expect("Data Classification layer should exist");
    
    // Verify the layer properties match the CSV
    assert_eq!(data_classification_layer.component_mechanism, "Classify data: public / internal / confidential / restricted");
    assert_eq!(data_classification_layer.goal, "Know which data needs strong controls");
    assert_eq!(data_classification_layer.evidence_telemetry, "Data inventory with labels");
    
    // Test the actual implementation...
}
```

## Testing

The implementation includes comprehensive unit tests that validate all functionality:

1. `test_data_classification_enum` - Tests the DataClassification enum functionality
2. `test_classified_data_asset` - Tests the ClassifiedDataAsset struct
3. `test_data_inventory` - Tests the DataInventory functionality
4. `test_data_classification_manager` - Tests the DataClassificationManager
5. `test_csv_requirement_classification_tiers` - Tests the specific CSV requirements
6. `test_data_classification_integration` - Integration tests

All tests can be run with:

```bash
cargo test -p security_layers --test data_security_classification_validation
```

## Evidence and Telemetry

The implementation provides the required "Data inventory with labels" evidence through the `generate_telemetry_report()` method, which produces output like:

```
Data Inventory with Labels:
Total Assets: 4
Classification Counts:
  confidential: 1
  public: 1
  restricted: 1
  internal: 1

Asset Details:
  ID: market-data-001, Name: Public Market Data, Classification: public, Location: database/market, Owner: data-team@example.com
  ID: user-wallets-001, Name: User Wallet Addresses, Classification: confidential, Location: database/users/wallets, Owner: security-team@example.com
  ID: admin-keys-001, Name: Administrative Private Keys, Classification: restricted, Location: vault/admin, Owner: security-team@example.com
  ID: process-doc-001, Name: Internal Trading Process, Classification: internal, Location: docs/internal, Owner: operations@example.com
```

This telemetry clearly shows which data assets exist, their classifications, and where they are stored, enabling security teams to "know which data needs strong controls."

## Security Benefits

1. **Clear Data Governance** - Explicit classification of all data assets
2. **Access Control Enforcement** - Different security controls can be applied based on classification
3. **Audit and Compliance** - Comprehensive inventory for regulatory compliance
4. **Risk Management** - Identification of high-value data assets requiring additional protection
5. **Incident Response** - Rapid identification of affected data in security incidents

## Future Enhancements

Potential future enhancements could include:

1. Automated classification based on data content analysis
2. Integration with data loss prevention (DLP) systems
3. Policy enforcement based on classification levels
4. Integration with encryption systems for automatic protection based on classification
5. Export functionality for compliance reporting