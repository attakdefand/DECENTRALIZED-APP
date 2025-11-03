// Vendor Key Management Policies
// Controls key management for third-party vendors

// Define vendor key management entities
entity VendorKeyManagement;
entity VendorKeyPolicy in VendorKeyManagement;
entity VendorKeyRotation in VendorKeyManagement;
entity VendorKeyHealthCheck in VendorKeyManagement;

// Define vendor key types
entity VendorKeyType::APIKey;
entity VendorKeyType::DatabaseCredential;
entity VendorKeyType::TLSCertificate;
entity VendorKeyType::SSHKey;
entity VendorKeyType::DataEncryptionKey;
entity VendorKeyType::KeyEncryptionKey;
entity VendorKeyType::ServiceAccountKey;

// Define vendor key management actions
action "manage_keys" appliesTo {
  principal: [Vendor, VendorUser, VendorTeam],
  resource: [VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
};

action "rotate_keys" appliesTo {
  principal: [Vendor, VendorUser, VendorTeam],
  resource: [VendorKeyRotation]
};

action "check_key_health" appliesTo {
  principal: [Vendor, VendorUser, VendorTeam],
  resource: [VendorKeyHealthCheck]
};

action "audit_key_management" appliesTo {
  principal: [Auditor, SecurityTeam],
  resource: [VendorKeyManagement, VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
};

// Vendors must comply with key rotation policies
permit (
  principal in Vendor,
  action == Action::"rotate_keys",
  resource in VendorKeyRotation
) when {
  principal.vendor_key_policy_compliance == true
};

// Vendors with non-compliant key rotation are blocked
forbid (
  principal in Vendor,
  action in [Action::"manage_keys", Action::"rotate_keys"],
  resource in [VendorKeyManagement, VendorKeyRotation]
) when {
  principal.vendor_key_rotation_compliance < 100
};

// Vendors must pass key health checks
permit (
  principal in Vendor,
  action == Action::"check_key_health",
  resource in VendorKeyHealthCheck
) when {
  principal.vendor_key_health_checks_pass == true
};

// Vendors with failed key health checks are blocked
forbid (
  principal in Vendor,
  action in [Action::"manage_keys", Action::"check_key_health"],
  resource in [VendorKeyManagement, VendorKeyHealthCheck]
) when {
  principal.vendor_key_health_checks_fail == true
};

// Vendors must have valid key management policies
forbid (
  principal in Vendor,
  action in [Action::"manage_keys", Action::"rotate_keys", Action::"check_key_health"],
  resource in [VendorKeyManagement, VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
) when {
  principal.vendor_key_policy_status != "active" || principal.vendor_key_policy_expiry < context.current_time
};

// Vendors must perform key rotations within required timeframes
forbid (
  principal in Vendor,
  action in [Action::"manage_keys", Action::"rotate_keys"],
  resource in [VendorKeyManagement, VendorKeyRotation]
) when {
  principal.last_key_rotation == null || principal.last_key_rotation < (context.current_time - principal.key_rotation_interval)
};

// Auditors can audit vendor key management
permit (
  principal in Auditor,
  action == Action::"audit_key_management",
  resource in [VendorKeyManagement, VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
);

// Security team can audit vendor key management
permit (
  principal in SecurityTeam,
  action == Action::"audit_key_management",
  resource in [VendorKeyManagement, VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
);

// Vendors with clean key management records get extended access
permit (
  principal in Vendor,
  action in [Action::"manage_keys", Action::"rotate_keys", Action::"check_key_health"],
  resource in [VendorKeyManagement, VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
) when {
  principal.vendor_key_rotation_compliance == 100 && 
  principal.vendor_key_health_checks_pass == true &&
  principal.vendor_key_policy_compliance == true
};

// Emergency key rotation for vendors during incidents
permit (
  principal in Vendor,
  action == Action::"rotate_keys",
  resource in VendorKeyRotation
) when {
  context.incident_declared == true && principal.emergency_contact == true
};

// Restrict key management access based on vendor role
permit (
  principal in VendorRole::Admin,
  action in [Action::"manage_keys", Action::"rotate_keys", Action::"check_key_health"],
  resource in [VendorKeyManagement, VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
);

permit (
  principal in VendorRole::Security,
  action in [Action::"manage_keys", Action::"rotate_keys", Action::"check_key_health"],
  resource in [VendorKeyManagement, VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
);

forbid (
  principal in VendorRole::ReadOnly,
  action in [Action::"manage_keys", Action::"rotate_keys", Action::"check_key_health"],
  resource in [VendorKeyManagement, VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
);

// Time-based restrictions for key management operations
forbid (
  principal in Vendor,
  action in [Action::"manage_keys", Action::"rotate_keys", Action::"check_key_health"],
  resource in [VendorKeyManagement, VendorKeyPolicy, VendorKeyRotation, VendorKeyHealthCheck]
) when {
  // Vendors can only perform key management during business hours (9 AM to 5 PM UTC)
  context.current_time.hour < 9 || context.current_time.hour >= 17
} unless {
  principal.role == "vendor_oncall" || principal.emergency_access == true
};