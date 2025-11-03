// Vendor Access Policies
// Controls access for third-party vendors to system resources

// Define vendor-specific entities
entity Vendor;
entity VendorUser in Vendor;
entity VendorTeam in Vendor;

// Define vendor access levels
entity VendorAccessLevel::Read;
entity VendorAccessLevel::Write;
entity VendorAccessLevel::Admin;
entity VendorAccessLevel::Root;

// Define vendor resource types
entity VendorResource::API;
entity VendorResource::Database;
entity VendorResource::FileSystem;
entity VendorResource::Network;
entity VendorResource::Application;

// Define vendor-specific actions
action "vendor_read" appliesTo {
  principal: [VendorUser, VendorTeam],
  resource: [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
};

action "vendor_write" appliesTo {
  principal: [VendorUser, VendorTeam],
  resource: [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
};

action "vendor_admin" appliesTo {
  principal: [VendorUser, VendorTeam],
  resource: [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
};

action "vendor_delete" appliesTo {
  principal: [VendorUser, VendorTeam],
  resource: [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
};

// Vendor administrators can perform any action on any vendor resource
permit (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.role == "vendor_admin"
};

// Vendor developers can read and write to vendor resources
permit (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.role == "vendor_developer"
};

// Vendor read-only users can only read vendor resources
permit (
  principal in Vendor,
  action == Action::"vendor_read",
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.role == "vendor_readonly"
};

// Restrict vendor access based on resource type and access level
forbid (
  principal in Vendor,
  action in [Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in VendorResource::Database
) when {
  principal.access_level == VendorAccessLevel::Read
};

forbid (
  principal in Vendor,
  action in [Action::"vendor_admin", Action::"vendor_delete"],
  resource in VendorResource::Database
) when {
  principal.access_level == VendorAccessLevel::Write
};

// Restrict access to sensitive vendor resources
forbid (
  principal in Vendor,
  action in [Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in VendorResource::FileSystem
) when {
  principal.role != "vendor_admin" && principal.role != "vendor_security"
};

// Time-based access restrictions for vendors
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  // Vendors can only access resources during business hours (9 AM to 5 PM UTC)
  context.current_time.hour < 9 || context.current_time.hour >= 17
} unless {
  principal.role == "vendor_oncall" || principal.emergency_access == true
};

// Restrict vendor access based on IP address ranges
permit (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write"],
  resource in [VendorResource::API, VendorResource::Database]
) when {
  principal.ip_address in context.allowed_vendor_ips
};

forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  !(principal.ip_address in context.allowed_vendor_ips)
};