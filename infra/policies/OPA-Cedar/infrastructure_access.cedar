// Infrastructure Access Policies
// Controls access to cloud and infrastructure resources

// Administrators can perform any action on any infrastructure resource
permit (
  principal in Administrators,
  action in [Action::"read", Action::"write", Action::"delete", Action::"admin"],
  resource in Infrastructure
);

// Operations team can manage infrastructure resources
permit (
  principal in OperationsTeam,
  action in [Action::"read", Action::"write", Action::"delete"],
  resource in Infrastructure
) when {
  // Operations team can only manage resources in their assigned environments
  resource.environment in principal.assigned_environments
};

// Developers can read infrastructure resources for debugging
permit (
  principal in Developers,
  action == Action::"read",
  resource in Infrastructure
) when {
  resource.environment == "development" || resource.environment == "staging"
};

// Security team can read infrastructure resources for security monitoring
permit (
  principal in SecurityTeam,
  action == Action::"read",
  resource in Infrastructure
);

// External contributors have no access to infrastructure resources
forbid (
  principal in ExternalContributors,
  action in [Action::"read", Action::"write", Action::"delete", Action::"admin"],
  resource in Infrastructure
);

// Emergency access for administrators during incidents
permit (
  principal in Administrators,
  action in [Action::"read", Action::"write", Action::"delete"],
  resource in Infrastructure
) when {
  // Only allowed during declared incidents
  context.incident_declared == true
};