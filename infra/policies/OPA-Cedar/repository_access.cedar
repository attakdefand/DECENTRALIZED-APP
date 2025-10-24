// Repository Access Policies
// Controls access to code repositories

// Administrators can perform any action on any repository
permit (
  principal in Administrators,
  action in [Action::"read", Action::"write", Action::"delete", Action::"admin"],
  resource in Repository
);

// Developers can read and write to repositories
permit (
  principal in Developers,
  action in [Action::"read", Action::"write"],
  resource in Repository
) when {
  // Developers can only write to repositories they own or are assigned to
  resource.owner == principal || resource in principal.assigned_repositories
};

// External contributors can only read public repositories
permit (
  principal in ExternalContributors,
  action == Action::"read",
  resource in Repository
) when {
  resource.visibility == "public"
};

// Security team can read all repositories for security reviews
permit (
  principal in SecurityTeam,
  action == Action::"read",
  resource in Repository
);

// Operations team can read repositories for infrastructure purposes
permit (
  principal in OperationsTeam,
  action == Action::"read",
  resource in Repository
) when {
  resource.type == "infrastructure"
};

// QA team can read repositories for testing purposes
permit (
  principal in QATeam,
  action == Action::"read",
  resource in Repository
) when {
  resource.environment == "testing" || resource.environment == "staging"
};