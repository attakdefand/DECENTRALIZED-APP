// Test policies for validation
// These policies are used for testing the policy engine

// Test entities
entity TestUser in [Developers, OperationsTeam];
entity TestRepository in Repository;
entity TestInfrastructure in Infrastructure;
entity TestDatabase in Data;

// Test actions
action "test_read" appliesTo {
  principal: [User, Team],
  resource: [Repository, Infrastructure, Data]
};

action "test_write" appliesTo {
  principal: [User, Team],
  resource: [Repository, Infrastructure, Data]
};

// Test policy - developers can read test repository
permit (
  principal in Developers,
  action == Action::"test_read",
  resource in Repository
);

// Test policy - operations can write to infrastructure
permit (
  principal in OperationsTeam,
  action == Action::"test_write",
  resource in Infrastructure
);