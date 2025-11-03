// Vendor RBAC Policies
// Role-based access control for vendor users and teams

// Define vendor roles
entity VendorRole;
entity VendorRole::Admin in VendorRole;
entity VendorRole::Developer in VendorRole;
entity VendorRole::Analyst in VendorRole;
entity VendorRole::Support in VendorRole;
entity VendorRole::ReadOnly in VendorRole;

// Define vendor permissions
entity VendorPermission;
entity VendorPermission::ReadAPI in VendorPermission;
entity VendorPermission::WriteAPI in VendorPermission;
entity VendorPermission::AdminAPI in VendorPermission;
entity VendorPermission::ReadDatabase in VendorPermission;
entity VendorPermission::WriteDatabase in VendorPermission;
entity VendorPermission::AdminDatabase in VendorPermission;
entity VendorPermission::ReadFileSystem in VendorPermission;
entity VendorPermission::WriteFileSystem in VendorPermission;
entity VendorPermission::AdminFileSystem in VendorPermission;

// Map roles to permissions
// Vendor Admin role - full access
permit (
  principal in VendorRole::Admin,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
);

// Vendor Developer role - read and write access
permit (
  principal in VendorRole::Developer,
  action in [Action::"vendor_read", Action::"vendor_write"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem]
) when {
  resource.type != "critical"
};

// Vendor Analyst role - read-only access to APIs and databases
permit (
  principal in VendorRole::Analyst,
  action == Action::"vendor_read",
  resource in [VendorResource::API, VendorResource::Database]
);

// Vendor Support role - limited access for support purposes
permit (
  principal in VendorRole::Support,
  action in [Action::"vendor_read", Action::"vendor_write"],
  resource in VendorResource::API
) when {
  resource.purpose == "support"
};

// Vendor ReadOnly role - read-only access to non-sensitive resources
permit (
  principal in VendorRole::ReadOnly,
  action == Action::"vendor_read",
  resource in [VendorResource::API, VendorResource::FileSystem]
) when {
  resource.sensitivity != "high" && resource.sensitivity != "critical"
};

// Separation of Duties (SoD) constraints for vendors
// Prevent vendor users from having both admin and support roles
forbid (
  principal in VendorUser,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.roles.contains(VendorRole::Admin) && principal.roles.contains(VendorRole::Support)
};

// Prevent vendor users from having both developer and analyst roles for critical systems
forbid (
  principal in VendorUser,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin"],
  resource in VendorResource::Database
) when {
  resource.sensitivity == "critical" && 
  principal.roles.contains(VendorRole::Developer) && 
  principal.roles.contains(VendorRole::Analyst)
};

// Role expiration and review constraints
forbid (
  principal in VendorUser,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.role_expiry < context.current_time
};

// Role assignment constraints - only authorized personnel can assign vendor roles
permit (
  principal in SecurityTeam,
  action in [Action::"assign_role", Action::"revoke_role"],
  resource in VendorUser
);

permit (
  principal in VendorAdmin,
  action in [Action::"assign_role", Action::"revoke_role"],
  resource in VendorUser
) when {
  principal.vendor_id == resource.vendor_id
};

forbid (
  principal in VendorUser,
  action in [Action::"assign_role", Action::"revoke_role"],
  resource in VendorUser
) unless {
  principal.id == resource.id // Users can modify their own roles in some cases
};

// Vendor role inheritance
permit (
  principal in VendorTeam,
  action in [Action::"vendor_read", Action::"vendor_write"],
  resource in VendorResource::API
) when {
  resource.team_id == principal.id
};

// Audit vendor role assignments
permit (
  principal in Auditor,
  action == Action::"audit_roles",
  resource in [VendorUser, VendorTeam]
);

// Security team can audit vendor role assignments
permit (
  principal in SecurityTeam,
  action == Action::"audit_roles",
  resource in [VendorUser, VendorTeam]
);