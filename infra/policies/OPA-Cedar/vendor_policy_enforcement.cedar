// Vendor Policy Enforcement Policies
// Enforces vendor security policies and compliance requirements with allow/deny lists and rate classes

// Define vendor policy entities
entity VendorPolicy;
entity VendorComplianceRequirement in VendorPolicy;
entity VendorSecurityControl in VendorPolicy;
entity VendorAllowDenyList in VendorPolicy;
entity VendorRateClass in VendorPolicy;

// Define vendor policy actions
action "check_compliance" appliesTo {
  principal: [Vendor, VendorUser, VendorTeam],
  resource: [VendorComplianceRequirement, VendorSecurityControl]
};

action "enforce_policy" appliesTo {
  principal: [Vendor, VendorUser, VendorTeam],
  resource: [VendorPolicy, VendorComplianceRequirement, VendorSecurityControl, VendorAllowDenyList, VendorRateClass]
};

action "audit_vendor" appliesTo {
  principal: [Auditor, SecurityTeam],
  resource: [Vendor, VendorUser, VendorTeam, VendorPolicy]
};

action "manage_allow_deny_list" appliesTo {
  principal: [SecurityTeam, VendorAdmin],
  resource: [VendorAllowDenyList]
};

action "apply_rate_class" appliesTo {
  principal: [SecurityTeam, VendorAdmin],
  resource: [VendorRateClass]
};

// Vendors must comply with all security requirements
permit (
  principal in Vendor,
  action == Action::"check_compliance",
  resource in VendorComplianceRequirement
) when {
  principal.compliance_status == "current"
};

// Vendors must have active security controls
permit (
  principal in Vendor,
  action == Action::"enforce_policy",
  resource in VendorSecurityControl
) when {
  principal.security_controls_active == true
};

// Vendors with expired compliance must be blocked
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.compliance_status == "expired" || principal.compliance_expiry < context.current_time
};

// Vendors with critical security violations must be blocked
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.security_violations > 0 && principal.critical_violations > 0
};

// Vendors with high-risk security violations have limited access
forbid (
  principal in Vendor,
  action in [Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.security_violations > 0 && principal.high_risk_violations > 0
};

// Vendors must have valid contracts
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.contract_status != "active" || principal.contract_expiry < context.current_time
};

// Vendors must pass periodic security assessments
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.last_assessment == null || principal.last_assessment < (context.current_time - 90 * 24 * 60 * 60) // 90 days
};

// Auditors can audit vendor compliance
permit (
  principal in Auditor,
  action == Action::"audit_vendor",
  resource in [Vendor, VendorUser, VendorTeam]
);

// Security team can audit vendor compliance
permit (
  principal in SecurityTeam,
  action == Action::"audit_vendor",
  resource in [Vendor, VendorUser, VendorTeam]
);

// Vendors with clean security records get extended access
permit (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.security_violations == 0 && principal.compliance_status == "current"
};

// Emergency access for vendors during incidents
permit (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write"],
  resource in [VendorResource::API, VendorResource::Database]
) when {
  context.incident_declared == true && principal.emergency_contact == true
};

// Allow/Deny List Enforcement
// Vendors can only access domains in the allow list
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  !(context.requested_domain in context.vendor_domain_allow_list)
};

// Vendors are blocked from accessing domains in the deny list
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  context.requested_domain in context.vendor_domain_deny_list
};

// Vendors can only access from IP addresses in the allow list
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  !(principal.ip_address in context.vendor_ip_allow_list)
};

// Vendors are blocked from accessing from IP addresses in the deny list
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.ip_address in context.vendor_ip_deny_list
};

// Rate Class Enforcement
// Apply rate limits based on vendor tier
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  context.request_rate > principal.vendor_rate_limit
};

// API rate limits
forbid (
  principal in Vendor,
  action == Action::"vendor_read",
  resource in VendorResource::API
) when {
  context.api_request_count > principal.api_rate_limit
};

// Network bandwidth limits
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write"],
  resource in [VendorResource::Network, VendorResource::FileSystem]
) when {
  context.bandwidth_usage > principal.bandwidth_limit
};

// Concurrent session limits
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.active_sessions > principal.session_limit
};

// Policy Coverage Enforcement
// Vendors must maintain minimum policy coverage
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.policy_coverage_pct < 95
};

// Vendors with policy violations are blocked
forbid (
  principal in Vendor,
  action in [Action::"vendor_read", Action::"vendor_write", Action::"vendor_admin", Action::"vendor_delete"],
  resource in [VendorResource::API, VendorResource::Database, VendorResource::FileSystem, VendorResource::Network, VendorResource::Application]
) when {
  principal.unresolved_policy_violations > 0
};

// Security team can manage allow/deny lists
permit (
  principal in SecurityTeam,
  action == Action::"manage_allow_deny_list",
  resource in VendorAllowDenyList
);

// Vendor admins can manage their own allow/deny lists with approval
permit (
  principal in VendorAdmin,
  action == Action::"manage_allow_deny_list",
  resource in VendorAllowDenyList
) when {
  principal.vendor_id == resource.vendor_id
};

// Security team can apply rate classes
permit (
  principal in SecurityTeam,
  action == Action::"apply_rate_class",
  resource in VendorRateClass
);

// Vendor admins can apply rate classes to their own resources with approval
permit (
  principal in VendorAdmin,
  action == Action::"apply_rate_class",
  resource in VendorRateClass
) when {
  principal.vendor_id == resource.vendor_id
};