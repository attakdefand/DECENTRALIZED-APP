// Data Access Policies
// Controls access to sensitive data resources

// Define data classification levels
entity DataClassification::Public;
entity DataClassification::Internal;
entity DataClassification::Confidential;
entity DataClassification::Restricted;

// Administrators can access all data classifications
permit (
  principal in Administrators,
  action in [Action::"read", Action::"write", Action::"delete", Action::"admin"],
  resource in Data
);

// Developers can access Public and Internal data
permit (
  principal in Developers,
  action in [Action::"read", Action::"write"],
  resource in Data
) when {
  resource.classification in [DataClassification::Public, DataClassification::Internal]
};

// Operations team can access all data for operational purposes
permit (
  principal in OperationsTeam,
  action in [Action::"read", Action::"write", Action::"delete"],
  resource in Data
) when {
  // Only in their assigned environments
  resource.environment in principal.assigned_environments
};

// Security team can access all data for security monitoring
permit (
  principal in SecurityTeam,
  action in [Action::"read"],
  resource in Data
);

// QA team can access test data only
permit (
  principal in QATeam,
  action in [Action::"read", Action::"write"],
  resource in Data
) when {
  resource.environment == "testing" && 
  resource.classification in [DataClassification::Public, DataClassification::Internal]
};

// External contributors can only access Public data
permit (
  principal in ExternalContributors,
  action == Action::"read",
  resource in Data
) when {
  resource.classification == DataClassification::Public
};

// Restrict access to Confidential and Restricted data
forbid (
  principal in [ExternalContributors, QATeam],
  action in [Action::"read", Action::"write", Action::"delete", Action::"admin"],
  resource in Data
) when {
  resource.classification in [DataClassification::Confidential, DataClassification::Restricted]
};

// Additional restrictions for write access to Confidential data
forbid (
  principal in Developers,
  action in [Action::"write", Action::"delete", Action::"admin"],
  resource in Data
) when {
  resource.classification == DataClassification::Confidential
};

// Encryption requirement for Restricted data
permit (
  principal,
  action in [Action::"read", Action::"write"],
  resource in Data
) when {
  resource.classification == DataClassification::Restricted &&
  context.encryption_enabled == true
};